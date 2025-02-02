use bitflags::bitflags;
use stormlib_sys;

bitflags! {
  pub struct OpenArchiveFlags: u32 {
    /// The MPQ is plain linear file. The file can have a block bitmap at the end, indicating that some file blocks may be missing. This is the default value.
    const STREAM_PROVIDER_FLAT = stormlib_sys::STREAM_PROVIDER_FLAT;
    /// The MPQ is stored in partial file. Partial files were used by trial version of World of Warcraft (build 10958 - 11685).
    const STREAM_PROVIDER_PARTIAL = stormlib_sys::STREAM_PROVIDER_PARTIAL;
    /// The MPQ is encrypted (.MPQE). Encrypted MPQs are used by Starcraft II and Diablo III installers. StormLib attempts to use all known keys. If no key can be used for decrypting the MPQ, the open operation fails.
    const STREAM_PROVIDER_MPQE = stormlib_sys::STREAM_PROVIDER_MPQE;
    /// The MPQ divided to multiple blocks and multiple files. Size of one block is 0x4000 bytes, maximum number of blocks per file is 0x2000. Each block is followed by MD5 hash in plain ANSI text form. If the total number of blocks in the archive is greater than 0x2000, then the archive is split into multiple files. These files have decimal numeric extensions in ascending order (.MPQ.0, .MPQ.1, .MPQ.2 and so on).
    const STREAM_PROVIDER_BLOCK4 = stormlib_sys::STREAM_PROVIDER_BLOCK4;

    /// The MPQ is in local file. This is the default value.
    const BASE_PROVIDER_FILE = stormlib_sys::BASE_PROVIDER_FILE;
    /// The MPQ is in local file. Stormlib will attempt to map the file into memory. This speeds up the MPQ operations (reading, verifying), but has size and operating system limitations.
    const BASE_PROVIDER_MAP = stormlib_sys::BASE_PROVIDER_MAP;
    /// The MPQ is on a web server available via HTTP protocol. The server must support random access. Only supported in Windows.
    const BASE_PROVIDER_HTTP = stormlib_sys::BASE_PROVIDER_HTTP;

    /// This flag causes the file to be open read-only. This flag is automatically set for partial and encrypted MPQs, and also for all MPQs that are not open from BASE_PROVIDER_FILE.
    const STREAM_FLAG_READ_ONLY = stormlib_sys::STREAM_FLAG_READ_ONLY;
    /// This flag causes the writable MPQ being open for write share. Use with caution. If two applications write to an open MPQ simultaneously, the MPQ data get corrupted.
    const STREAM_FLAG_WRITE_SHARE = stormlib_sys::STREAM_FLAG_WRITE_SHARE;
    /// This flag tells the file stream handler to respect a file bitmap. File bitmap is used by MPQs whose file blocks are downloaded on demand by the game.
    const STREAM_FLAG_USE_BITMAP = stormlib_sys::STREAM_FLAG_USE_BITMAP;

    /// Don't read the internal listfile.
    const MPQ_OPEN_NO_LISTFILE = stormlib_sys::MPQ_OPEN_NO_LISTFILE;
    /// Don't open the "(attributes)" file.
    const MPQ_OPEN_NO_ATTRIBUTES = stormlib_sys::MPQ_OPEN_NO_ATTRIBUTES;
    /// Do not search the header at 0x200 byte offsets.
    const MPQ_OPEN_NO_HEADER_SEARCH = stormlib_sys::MPQ_OPEN_NO_HEADER_SEARCH;
    /// Forces the MPQ to be open as MPQ format 1.0, ignoring "wFormatVersion" variable in the header.
    const MPQ_OPEN_FORCE_MPQ_V1 = stormlib_sys::MPQ_OPEN_FORCE_MPQ_V1;
    /// SFileReadFile will check CRC of each file sector on any file in the archive until the archive is closed.
    const MPQ_OPEN_CHECK_SECTOR_CRC = stormlib_sys::MPQ_OPEN_CHECK_SECTOR_CRC;
  }
}

bitflags! {
  pub struct CreateArchiveFlags: u32 {
    /// The newly created archive will have (listfile) present.
    ///
    /// Note that all archives created by SFileCreateArchive have listfile present due to compatibility reasons.
    const MPQ_CREATE_LISTFILE = stormlib_sys::MPQ_CREATE_LISTFILE;
    /// The newly created archive will have additional attributes in (attributes) file.
    const MPQ_CREATE_ATTRIBUTES = stormlib_sys::MPQ_CREATE_ATTRIBUTES;
    /// The newly created archive will be signed with weak digital signature (the "(signature) file).
    const MPQ_CREATE_SIGNATURE = stormlib_sys::MPQ_CREATE_SIGNATURE;

    /// The function creates a MPQ version 1.0 (up to 4 GB). This is the default value
    const MPQ_CREATE_ARCHIVE_V1 = stormlib_sys::MPQ_CREATE_ARCHIVE_V1;
    /// The function creates a MPQ version 2.0 (supports MPQ of size greater than 4 GB).
    const MPQ_CREATE_ARCHIVE_V2 = stormlib_sys::MPQ_CREATE_ARCHIVE_V2;
    /// The function creates a MPQ version 3.0 (introduced in WoW-Cataclysm Beta).
    const MPQ_CREATE_ARCHIVE_V3 = stormlib_sys::MPQ_CREATE_ARCHIVE_V3;
    /// The function creates a MPQ version 4.0 (used in WoW-Cataclysm).
    const MPQ_CREATE_ARCHIVE_V4 = stormlib_sys::MPQ_CREATE_ARCHIVE_V4;
  }
}

bitflags! {
  pub struct CreateFileFlags: u32 {
    /// The file will be compressed using IMPLODE compression method. This flag cannot be used together with MPQ_FILE_COMPRESS. If this flag is present, then the dwCompression and dwCompressionNext parameters are ignored. This flag is obsolete and was used only in Diablo I.
    const MPQ_FILE_IMPLODE = stormlib_sys::MPQ_FILE_IMPLODE;
    /// The file will be compressed. This flag cannot be used together with MPQ_FILE_IMPLODE.
    const MPQ_FILE_COMPRESS = stormlib_sys::MPQ_FILE_COMPRESS;
    /// The file will be stored as encrypted.
    const MPQ_FILE_ENCRYPTED = stormlib_sys::MPQ_FILE_ENCRYPTED;
    /// The file's encryption key will be adjusted according to file size in the archive. This flag must be used together with MPQ_FILE_ENCRYPTED.
    const MPQ_FILE_FIX_KEY = stormlib_sys::MPQ_FILE_FIX_KEY;
    /// The file will have the deletion marker.
    const MPQ_FILE_DELETE_MARKER = stormlib_sys::MPQ_FILE_DELETE_MARKER;
    /// The file will have CRC for each file sector. Ignored if the file is not compressed or if the file is stored as single unit.
    const MPQ_FILE_SECTOR_CRC = stormlib_sys::MPQ_FILE_SECTOR_CRC;
    /// The file will be added as single unit. Files stored as single unit cannot be encrypted, because Blizzard doesn't support them.
    const MPQ_FILE_SINGLE_UNIT = stormlib_sys::MPQ_FILE_SINGLE_UNIT;
    /// If this flag is specified and the file is already in the MPQ, it will be replaced.
    const MPQ_FILE_REPLACEEXISTING = stormlib_sys::MPQ_FILE_REPLACEEXISTING;
  }
}

bitflags! {
  pub struct CompressionFlags: u32 {
    /// Use Huffman compression. This bit can only be combined with MPQ_COMPRESSION_ADPCM_MONO or MPQ_COMPRESSION_ADPCM_STEREO.
    const MPQ_COMPRESSION_HUFFMANN = stormlib_sys::MPQ_COMPRESSION_HUFFMANN;
    /// Use ZLIB compression library. This bit cannot be combined with MPQ_COMPRESSION_BZIP2 or MPQ_COMPRESSION_LZMA.
    const MPQ_COMPRESSION_ZLIB = stormlib_sys::MPQ_COMPRESSION_ZLIB;
    /// Use Pkware Data Compression Library. This bit cannot be combined with MPQ_COMPRESSION_LZMA.
    const MPQ_COMPRESSION_PKWARE = stormlib_sys::MPQ_COMPRESSION_PKWARE;
    /// Use BZIP2 compression library. This bit cannot be combined with MPQ_COMPRESSION_ZLIB or MPQ_COMPRESSION_LZMA.
    const MPQ_COMPRESSION_BZIP2 = stormlib_sys::MPQ_COMPRESSION_BZIP2;
    /// Use SPARSE compression. This bit cannot be combined with MPQ_COMPRESSION_LZMA.
    const MPQ_COMPRESSION_SPARSE = stormlib_sys::MPQ_COMPRESSION_SPARSE;
    /// Use IMA ADPCM compression for 1-channel (mono) WAVE files. This bit can only be combined with MPQ_COMPRESSION_HUFFMANN. This is lossy compression and should only be used for compressing WAVE files.
    const MPQ_COMPRESSION_ADPCM_MONO = stormlib_sys::MPQ_COMPRESSION_ADPCM_MONO;
    /// Use IMA ADPCM compression for 2-channel (stereo) WAVE files. This bit can only be combined with MPQ_COMPRESSION_HUFFMANN. This is lossy compression and should only be used for compressing WAVE files.
    const MPQ_COMPRESSION_ADPCM_STEREO = stormlib_sys::MPQ_COMPRESSION_ADPCM_STEREO;
    /// Use LZMA compression. This value can not be combined with any other compression method.
    const MPQ_COMPRESSION_LZMA = stormlib_sys::MPQ_COMPRESSION_LZMA;
  }
}
