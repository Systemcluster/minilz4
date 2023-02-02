use libc::{c_char, c_uint, c_void, size_t};

pub const LZ4F_VERSION: c_uint = 100;

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum BlockSize {
    Default  = 0,
    Max64KB  = 4,
    Max256KB = 5,
    Max1MB   = 6,
    Max4MB   = 7,
}
impl BlockSize {
    pub fn bytes(&self) -> usize {
        match self {
            BlockSize::Default => 64 * 1024,
            BlockSize::Max64KB => 64 * 1024,
            BlockSize::Max256KB => 256 * 1024,
            BlockSize::Max1MB => 1024 * 1024,
            BlockSize::Max4MB => 4096 * 1024,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum BlockMode {
    Linked      = 0,
    Independent = 1,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ContentChecksum {
    NoChecksum      = 0,
    ChecksumEnabled = 1,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum BlockChecksum {
    NoChecksum      = 0,
    ChecksumEnabled = 1,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum LZ4FrameType {
    LZ4Frame       = 0,
    SkippableFrame = 1,
}

#[repr(C)]
pub struct LZ4FFrameInfo {
    pub block_size_id:         BlockSize,
    pub block_mode:            BlockMode,
    pub content_checksum_flag: ContentChecksum,
    pub frame_type:            LZ4FrameType,
    pub content_size:          u64,
    pub dict_id:               u32,
    pub block_checksum_flag:   BlockChecksum,
}

#[repr(C)]
pub struct LZ4FPreferences {
    pub frame_info:        LZ4FFrameInfo,
    pub compression_level: c_uint,
    pub auto_flush:        c_uint,
    pub favor_dec_speed:   c_uint,
    pub reserved:          [c_uint; 3],
}

#[repr(C)]
pub struct LZ4FCompressOptions {
    pub stable_src: c_uint,
    pub reserved:   [c_uint; 3],
}

#[repr(C)]
pub struct LZ4FDecompressOptions {
    pub stable_dst: c_uint,
    pub reserved:   [c_uint; 3],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LZ4FCDict(pub *mut c_void);
unsafe impl Send for LZ4FCDict {}
unsafe impl Sync for LZ4FCDict {}

pub type LZ4FCompressionContextPtr = *mut c_void;
pub type LZ4FDecompressionContextPtr = *mut c_void;

pub type LZ4FErrorCode = size_t;

extern "C" {
    pub fn LZ4F_isError(code: size_t) -> c_uint;
    pub fn LZ4F_getErrorName(code: size_t) -> *const c_char;
    pub fn LZ4F_createCompressionContext(
        ctx: &mut LZ4FCompressionContextPtr, version: c_uint,
    ) -> LZ4FErrorCode;
    pub fn LZ4F_freeCompressionContext(ctx: LZ4FCompressionContextPtr) -> LZ4FErrorCode;
    pub fn LZ4F_compressBegin(
        ctx: LZ4FCompressionContextPtr, dstBuffer: *mut u8, dstMaxSize: size_t,
        preferencesPtr: *const LZ4FPreferences,
    ) -> LZ4FErrorCode;
    pub fn LZ4F_compressBound(
        srcSize: size_t, preferencesPtr: *const LZ4FPreferences,
    ) -> LZ4FErrorCode;
    pub fn LZ4F_compressUpdate(
        ctx: LZ4FCompressionContextPtr, dstBuffer: *mut u8, dstMaxSize: size_t,
        srcBuffer: *const u8, srcSize: size_t, compressOptionsPtr: *const LZ4FCompressOptions,
    ) -> size_t;
    pub fn LZ4F_flush(
        ctx: LZ4FCompressionContextPtr, dstBuffer: *mut u8, dstMaxSize: size_t,
        compressOptionsPtr: *const LZ4FCompressOptions,
    ) -> LZ4FErrorCode;
    pub fn LZ4F_compressEnd(
        ctx: LZ4FCompressionContextPtr, dstBuffer: *mut u8, dstMaxSize: size_t,
        compressOptionsPtr: *const LZ4FCompressOptions,
    ) -> LZ4FErrorCode;
    pub fn LZ4F_createDecompressionContext(
        ctx: &mut LZ4FDecompressionContextPtr, version: c_uint,
    ) -> LZ4FErrorCode;
    pub fn LZ4F_freeDecompressionContext(ctx: LZ4FDecompressionContextPtr) -> LZ4FErrorCode;
    pub fn LZ4F_decompress(
        ctx: LZ4FDecompressionContextPtr, dstBuffer: *mut u8, dstSizePtr: &mut size_t,
        srcBuffer: *const u8, srcSizePtr: &mut size_t, optionsPtr: *const LZ4FDecompressOptions,
    ) -> LZ4FErrorCode;
}
