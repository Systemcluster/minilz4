use crate::sys::*;

use std::{
    ffi::CStr,
    io::{Error as IOError, ErrorKind as IOErrorKind},
    ptr,
    result::{
        Result,
        Result::{Err, Ok},
    },
};

pub fn wrap_error(value: LZ4FErrorCode) -> Result<usize, IOError> {
    if unsafe { LZ4F_isError(value) } == 0 {
        Ok(value)
    } else {
        let error = unsafe { LZ4F_getErrorName(value) };
        let error = unsafe { CStr::from_ptr(error) };
        Err(IOError::new(
            IOErrorKind::Other,
            error.to_string_lossy().to_string(),
        ))
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LZ4FDecompressionContext(pub LZ4FDecompressionContextPtr);
unsafe impl Send for LZ4FDecompressionContext {}

impl LZ4FDecompressionContext {
    pub fn new() -> Result<LZ4FDecompressionContext, IOError> {
        let mut context = LZ4FDecompressionContext(ptr::null_mut());
        wrap_error(unsafe { LZ4F_createDecompressionContext(&mut context.0, LZ4F_VERSION) })?;
        Ok(context)
    }
}
impl Drop for LZ4FDecompressionContext {
    fn drop(&mut self) { unsafe { LZ4F_freeDecompressionContext(self.0) }; }
}

#[derive(Clone)]
#[repr(C)]
pub struct LZ4FCompressionContext(pub LZ4FCompressionContextPtr);
unsafe impl Send for LZ4FCompressionContext {}

impl LZ4FCompressionContext {
    pub fn new() -> Result<LZ4FCompressionContext, IOError> {
        let mut context = LZ4FCompressionContext(ptr::null_mut());
        wrap_error(unsafe { LZ4F_createCompressionContext(&mut context.0, LZ4F_VERSION) })?;
        Ok(context)
    }
}
impl Drop for LZ4FCompressionContext {
    fn drop(&mut self) { unsafe { LZ4F_freeCompressionContext(self.0) }; }
}
