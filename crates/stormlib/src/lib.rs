use std::ffi::*;
use std::path::Path;
use std::ptr;
use stormlib_sys::*;

#[macro_use]
mod util;

mod constants;
pub use constants::*;

pub mod error;
use error::*;

pub struct CreateFileOptions<'a> {
  path: &'a str,
  data: &'a Vec<u8>,
  flags: CreateFileFlags,
  mtime: u64,
  compression: CompressionFlags,
}

/// MPQ archive
#[derive(Debug)]
pub struct Archive {
  handle: HANDLE,
}

impl Archive {
  /// Creates new MPQ archive
  pub fn create<P: AsRef<Path>>(
    path: P,
    flags: CreateArchiveFlags,
    max_files_count: DWORD,
  ) -> Result<Self> {
    #[cfg(not(target_os = "windows"))]
    let cpath = {
      let pathstr = path.as_ref().to_str().ok_or_else(|| StormError::NonUtf8)?;
      CString::new(pathstr)?
    };
    #[cfg(target_os = "windows")]
    let cpath = {
      use widestring::U16CString;
      U16CString::from_os_str(path.as_ref())
        .map_err(|_| StormError::InteriorNul)?
        .into_vec()
    };

    let mut handle: HANDLE = ptr::null_mut();
    unsafe_try_call!(SFileCreateArchive(
      cpath.as_ptr(),
      flags.bits(),
      max_files_count,
      &mut handle
    ));

    Ok(Archive { handle })
  }

  /// Opens a MPQ archive
  pub fn open<P: AsRef<Path>>(path: P, flags: OpenArchiveFlags) -> Result<Self> {
    #[cfg(not(target_os = "windows"))]
    let cpath = {
      let pathstr = path.as_ref().to_str().ok_or_else(|| StormError::NonUtf8)?;
      CString::new(pathstr)?
    };
    #[cfg(target_os = "windows")]
    let cpath = {
      use widestring::U16CString;
      U16CString::from_os_str(path.as_ref())
        .map_err(|_| StormError::InteriorNul)?
        .into_vec()
    };

    let mut handle: HANDLE = ptr::null_mut();
    unsafe_try_call!(SFileOpenArchive(
      cpath.as_ptr(),
      0,
      flags.bits(),
      &mut handle
    ));

    Ok(Archive { handle })
  }

  /// Flushes in-memory changes to the archive on disk. This function is not necessary to call, as the archive will be flushed automatically when closed
  pub fn flush(&mut self) -> Result<()> {
    unsafe_try_call!(SFileFlushArchive(self.handle));
    Ok(())
  }

  /// Compacts the archive with an optional progress callback
  pub fn compact(&mut self, callback: Option<SFILE_COMPACT_CALLBACK>) -> Result<()> {
    if let Some(cb) = callback {
      unsafe_try_call!(SFileSetCompactCallback(self.handle, cb, ptr::null_mut()));
    }

    unsafe_try_call!(SFileCompactArchive(self.handle, ptr::null_mut(), false));

    // Reset the callback
    if callback.is_some() {
      unsafe_try_call!(SFileSetCompactCallback(self.handle, None, ptr::null_mut()));
    }

    Ok(())
  }

  /// Quick check if the file exists within MPQ archive, without opening it
  pub fn has_file(&mut self, path: &str) -> Result<bool> {
    let cpath = CString::new(path)?;
    unsafe {
      let r = SFileHasFile(self.handle, cpath.as_ptr());
      if !r {
        let err = GetLastError();
        if err != ERROR_FILE_NOT_FOUND {
          return Err(From::from(ErrorCode(err)));
        }
      }
      Ok(r)
    }
  }

  /// Creates a new file within the archive
  pub fn create_file<'a>(&'a mut self, opts: &'a CreateFileOptions) -> Result<()> {
    let cpath = CString::new(opts.path)?;

    let mut file_handle: HANDLE = ptr::null_mut();
    unsafe_try_call!(SFileCreateFile(
      self.handle,
      cpath.as_ptr(),
      opts.mtime,
      opts.data.len() as u32,
      0,
      opts.flags.bits(),
      &mut file_handle,
    ));

    unsafe_try_call!(SFileWriteFile(
      file_handle,
      opts.data.as_ptr() as *const _,
      opts.data.len() as u32,
      opts.compression.bits(),
    ));

    unsafe_try_call!(SFileFinishFile(file_handle));

    Ok(())
  }

  /// Opens a file from MPQ archive
  pub fn open_file<'a>(&'a mut self, path: &str) -> Result<File<'a>> {
    let mut file_handle: HANDLE = ptr::null_mut();
    let cpath = CString::new(path)?;

    unsafe_try_call!(SFileOpenFileEx(
      self.handle,
      cpath.as_ptr(),
      0,
      &mut file_handle
    ));

    Ok(File {
      archive: self,
      file_handle,
      size: None,
      need_reset: false,
    })
  }

  pub fn remove_file(self, path: &str) -> Result<bool> {
    let cpath = CString::new(path)?;
    unsafe {
      let r = SFileRemoveFile(self.handle, cpath.as_ptr(), 0);
      if !r {
        let err = GetLastError();
        if err != ERROR_FILE_NOT_FOUND {
          return Err(From::from(ErrorCode(err)));
        }
      }
      Ok(r)
    }
  }

  /// Searches for files within the archive. If `search_phrase` is `None`, all files will be returned
  pub fn search<'a>(&'a self, filter: Option<&str>) -> Result<Search<'a>> {
    let cfilter = CString::new(filter.unwrap_or("*"))?;
    Ok(Search {
      archive: self,
      filter: cfilter,
      find_handle: None,
    })
  }
}

impl std::ops::Drop for Archive {
  fn drop(&mut self) {
    unsafe {
      SFileCloseArchive(self.handle);
    }
  }
}

/// Opened file
#[derive(Debug)]
pub struct File<'a> {
  archive: &'a Archive,
  file_handle: HANDLE,
  size: Option<u64>,
  need_reset: bool,
}

impl<'a> File<'a> {
  /// Retrieves a size of the file within archive
  pub fn get_size(&mut self) -> Result<u64> {
    if let Some(size) = self.size.clone() {
      return Ok(size);
    }

    let mut high: DWORD = 0;
    let low = unsafe { SFileGetFileSize(self.file_handle, &mut high) };
    if low == SFILE_INVALID_SIZE {
      return Err(From::from(ErrorCode(unsafe { GetLastError() })));
    }
    let high = (high as u64) << 32;
    let size = high | (low as u64);
    self.size = Some(size);
    return Ok(size);
  }

  /// Reads all data from the file
  pub fn read_all(&mut self) -> Result<Vec<u8>> {
    if self.need_reset {
      unsafe {
        if SFileSetFilePointer(self.file_handle, 0, ptr::null_mut(), 0) == SFILE_INVALID_SIZE {
          return Err(From::from(ErrorCode(GetLastError())));
        }
      }
    }

    let size = self.get_size()?;
    let mut buf = Vec::<u8>::with_capacity(size as usize);
    buf.resize(buf.capacity(), 0);
    let mut read: DWORD = 0;
    self.need_reset = true;

    unsafe_try_call!(SFileReadFile(
      self.file_handle,
      std::mem::transmute(buf.as_mut_ptr()),
      size as u32,
      &mut read,
      ptr::null_mut(),
    ));

    if (read as u64) < size {
      buf.truncate(read as usize);
    }

    Ok(buf)
  }
}

impl<'a> std::ops::Drop for File<'a> {
  fn drop(&mut self) {
    unsafe {
      SFileCloseFile(self.file_handle);
    }
  }
}

/// Search iterator
#[derive(Debug)]
pub struct Search<'a> {
  archive: &'a Archive,
  filter: CString,
  find_handle: Option<HANDLE>,
}

impl<'a> Iterator for Search<'a> {
  type Item = SFILE_FIND_DATA;

  fn next(&mut self) -> Option<Self::Item> {
    let mut file_data: SFILE_FIND_DATA = unsafe { std::mem::zeroed() };

    if let Some(handle) = self.find_handle {
      let result = unsafe { SFileFindNextFile(handle, &mut file_data) };
      if result {
        return Some(file_data);
      }
    } else {
      let handle = unsafe {
        SFileFindFirstFile(
          self.archive.handle,
          self.filter.as_ptr(),
          &mut file_data,
          ptr::null_mut(),
        )
      };
      if !handle.is_null() {
        self.find_handle = Some(handle);
        return Some(file_data);
      }
    }

    None
  }
}

impl<'a> Drop for Search<'a> {
  fn drop(&mut self) {
    if let Some(handle) = self.find_handle {
      unsafe {
        SFileFindClose(handle);
      }
    }
  }
}

#[test]
fn test_read() {
  let mut archive = Archive::open(
    "../../samples/test_tft.w3x",
    OpenArchiveFlags::MPQ_OPEN_NO_LISTFILE | OpenArchiveFlags::MPQ_OPEN_NO_ATTRIBUTES,
  )
  .unwrap();

  assert_eq!(archive.has_file("invalid").unwrap(), false);
  assert_eq!(archive.has_file("war3map.j").unwrap(), true);
  let mut f = archive.open_file("war3map.j").unwrap();
  assert_eq!(f.get_size().unwrap(), 14115);
  assert_eq!(
    f.read_all().unwrap(),
    std::fs::read("../../samples/war3map.j").unwrap()
  );
}

#[test]
fn test_create_archive() {
  let archive_path = "../../samples/test_create_archive.mpq";
  let file_path = "test.txt";
  let file_data = b"Hello, MPQ!";
  let file_size = file_data.len() as u64;

  let result = std::panic::catch_unwind(|| {
    {
      // Create a new archive
      let mut archive =
        Archive::create(archive_path, CreateArchiveFlags::MPQ_CREATE_LISTFILE, 1000).unwrap();

      // Create a new file within the archive
      let opts = CreateFileOptions {
        path: file_path,
        data: &file_data.to_vec(),
        flags: CreateFileFlags::MPQ_FILE_COMPRESS,
        mtime: 0,
        compression: CompressionFlags::MPQ_COMPRESSION_ZLIB,
      };
      archive.create_file(&opts).unwrap();

      // Ensure the file exists within the archive
      assert_eq!(archive.has_file(file_path).unwrap(), true);
    }

    {
      // Reopen the archive
      let mut archive =
        Archive::open(archive_path, OpenArchiveFlags::STREAM_FLAG_READ_ONLY).unwrap();

      // Ensure the file exists within the archive
      assert_eq!(archive.has_file(file_path).unwrap(), true);
      assert_eq!(archive.has_file("missing").unwrap(), false);

      {
        // Search for the file within the archive
        let mut search = archive.search(Some(file_path)).unwrap();
        let search_result = search.next();
        assert!(search_result.is_some());
        let result_file_name = String::from_utf8(
          search_result
            .unwrap()
            .cFileName
            .iter()
            .map(|&x| x as u8)
            .collect(),
        )
        .unwrap();
        assert!(result_file_name.starts_with(file_path));
      }

      // Open the file and compare its size and data to the original data
      let mut file = archive.open_file(file_path).unwrap();
      assert_eq!(file.get_size().unwrap(), file_size);
      assert_eq!(file.read_all().unwrap(), file_data.to_vec());
    }
  });

  // Clean up
  std::fs::remove_file(archive_path).unwrap();

  // Propagate any panic that occurred during the test
  result.unwrap();
}

#[cfg(target_os = "windows")]
#[test]
fn test_read_unicode() {
  use std::os::windows::ffi::OsStringExt;
  use widestring::U16CString;
  let mut archive = Archive::open(
    OsString::from_wide(
      &U16CString::from_str("../../samples/中文.w3x")
        .unwrap()
        .into_vec(),
    ),
    OpenArchiveFlags::MPQ_OPEN_NO_LISTFILE | OpenArchiveFlags::MPQ_OPEN_NO_ATTRIBUTES,
  )
  .unwrap();
  let mut f = archive.open_file("war3map.j").unwrap();
  assert_eq!(
    f.read_all().unwrap(),
    std::fs::read("../../samples/war3map.j").unwrap()
  );
}

#[cfg(target_os = "macos")]
#[test]
fn test_read_utf8() {
  let mut archive = Archive::open(
    "../../samples/中文.w3x",
    OpenArchiveFlags::MPQ_OPEN_NO_LISTFILE | OpenArchiveFlags::MPQ_OPEN_NO_ATTRIBUTES,
  )
  .unwrap();
  let mut f = archive.open_file("war3map.j").unwrap();
  assert_eq!(
    f.read_all().unwrap(),
    std::fs::read("../../samples/war3map.j").unwrap()
  );
}
