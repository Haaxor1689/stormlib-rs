use bitflags::bitflags;
use stormlib_sys;

bitflags! {
  pub struct OpenArchiveFlags: u32 {
    const STREAM_PROVIDER_PARTIAL = stormlib_sys::STREAM_PROVIDER_PARTIAL;
    const STREAM_PROVIDER_MPQE = stormlib_sys::STREAM_PROVIDER_MPQE;
    const STREAM_PROVIDER_BLOCK4 = stormlib_sys::STREAM_PROVIDER_BLOCK4;
    const STREAM_PROVIDER_MASK = stormlib_sys::STREAM_PROVIDER_MASK;

    const BASE_PROVIDER_FILE = stormlib_sys::BASE_PROVIDER_FILE;
    const BASE_PROVIDER_MAP = stormlib_sys::BASE_PROVIDER_MAP ;
    const BASE_PROVIDER_HTTP = stormlib_sys::BASE_PROVIDER_HTTP;
    const BASE_PROVIDER_MASK = stormlib_sys::BASE_PROVIDER_MASK;

    const STREAM_FLAG_READ_ONLY = stormlib_sys::STREAM_FLAG_READ_ONLY;
    const STREAM_FLAG_WRITE_SHARE = stormlib_sys::STREAM_FLAG_WRITE_SHARE;

    const MPQ_OPEN_NO_LISTFILE = stormlib_sys::MPQ_OPEN_NO_LISTFILE;
    const MPQ_OPEN_NO_ATTRIBUTES = stormlib_sys::MPQ_OPEN_NO_ATTRIBUTES;
    const MPQ_OPEN_NO_HEADER_SEARCH = stormlib_sys::MPQ_OPEN_NO_HEADER_SEARCH;
    const MPQ_OPEN_FORCE_MPQ_V1 = stormlib_sys::MPQ_OPEN_FORCE_MPQ_V1;
    const MPQ_OPEN_CHECK_SECTOR_CRC = stormlib_sys::MPQ_OPEN_CHECK_SECTOR_CRC;
  }
}

bitflags! {
  pub struct CreateArchiveFlags: u32 {
    const MPQ_CREATE_LISTFILE = stormlib_sys::MPQ_CREATE_LISTFILE;
    const MPQ_CREATE_ATTRIBUTES = stormlib_sys::MPQ_CREATE_ATTRIBUTES;
    const MPQ_CREATE_SIGNATURE = stormlib_sys::MPQ_CREATE_SIGNATURE;

    const MPQ_CREATE_ARCHIVE_V1 = stormlib_sys::MPQ_CREATE_ARCHIVE_V1;
    const MPQ_CREATE_ARCHIVE_V2 = stormlib_sys::MPQ_CREATE_ARCHIVE_V2;
    const MPQ_CREATE_ARCHIVE_V3 = stormlib_sys::MPQ_CREATE_ARCHIVE_V3;
    const MPQ_CREATE_ARCHIVE_V4 = stormlib_sys::MPQ_CREATE_ARCHIVE_V4;
  }
}

bitflags! {
  pub struct CreateFileFlags: u32 {
    const MPQ_FILE_IMPLODE = stormlib_sys::MPQ_FILE_IMPLODE;
    const MPQ_FILE_COMPRESS = stormlib_sys::MPQ_FILE_COMPRESS;
    const MPQ_FILE_ENCRYPTED = stormlib_sys::MPQ_FILE_ENCRYPTED;
    const MPQ_FILE_FIX_KEY = stormlib_sys::MPQ_FILE_FIX_KEY;
    const MPQ_FILE_DELETE_MARKER = stormlib_sys::MPQ_FILE_DELETE_MARKER;
    const MPQ_FILE_SECTOR_CRC = stormlib_sys::MPQ_FILE_SECTOR_CRC;
    const MPQ_FILE_SINGLE_UNIT = stormlib_sys::MPQ_FILE_SINGLE_UNIT;
    const MPQ_FILE_REPLACEEXISTING = stormlib_sys::MPQ_FILE_REPLACEEXISTING;
  }
}

bitflags! {
  pub struct CompressionFlags: u32 {
    const MPQ_COMPRESSION_HUFFMANN = stormlib_sys::MPQ_COMPRESSION_HUFFMANN;
    const MPQ_COMPRESSION_ZLIB = stormlib_sys::MPQ_COMPRESSION_ZLIB;
    const MPQ_COMPRESSION_PKWARE = stormlib_sys::MPQ_COMPRESSION_PKWARE;
    const MPQ_COMPRESSION_BZIP2 = stormlib_sys::MPQ_COMPRESSION_BZIP2;
    const MPQ_COMPRESSION_SPARSE = stormlib_sys::MPQ_COMPRESSION_SPARSE;
    const MPQ_COMPRESSION_ADPCM_MONO = stormlib_sys::MPQ_COMPRESSION_ADPCM_MONO;
    const MPQ_COMPRESSION_ADPCM_STEREO = stormlib_sys::MPQ_COMPRESSION_ADPCM_STEREO;
    const MPQ_COMPRESSION_LZMA = stormlib_sys::MPQ_COMPRESSION_LZMA;
  }
}
