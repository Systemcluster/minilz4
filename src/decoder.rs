use crate::{context::*, sys::*};

pub use std::io::Read;
use std::{
    io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult},
    ptr,
};

use libc::size_t;

const BUFFER_SIZE: usize = 32 * 1024;

/// Decoder for LZ4 frame format data.
pub struct Decoder<R: Read> {
    context:  LZ4FDecompressionContext,
    reader:   R,
    buffer:   Box<[u8]>,
    position: usize,
    length:   usize,
    next:     usize,
}

impl<R: Read> Decoder<R> {
    pub fn new(reader: R) -> IOResult<Decoder<R>> {
        Ok(Decoder {
            reader,
            context: LZ4FDecompressionContext::new()?,
            buffer: Box::new([0; BUFFER_SIZE]),
            position: BUFFER_SIZE,
            length: BUFFER_SIZE,
            next: 11,
        })
    }

    pub fn finish(self) -> IOResult<R> {
        match self.next {
            0 => Ok(self.reader),
            _ => Err(IOError::new(
                IOErrorKind::Interrupted,
                "finish called before compressed stream was completely read",
            )),
        }
    }

    pub fn decode(mut self) -> IOResult<Vec<u8>> {
        let mut content = Vec::new();
        self.read_to_end(&mut content)?;
        Ok(content)
    }
}

/// Trait for decoding `Read` implementing objects.
pub trait Decode {
    fn decode(&mut self) -> IOResult<Vec<u8>>;
}

impl<R> Decode for R
where
    R: Read,
{
    fn decode(&mut self) -> IOResult<Vec<u8>> { Decoder::new(self)?.decode() }
}

impl<R: Read> Read for Decoder<R> {
    fn read(&mut self, buffer: &mut [u8]) -> IOResult<usize> {
        if self.next == 0 || buffer.len() == 0 {
            return Ok(0);
        }
        let mut dst_offset: usize = 0;
        while dst_offset == 0 {
            if self.position >= self.length {
                let need = match self.buffer.len() < self.next {
                    true => self.buffer.len(),
                    false => self.next,
                };
                self.length = self.reader.read(&mut self.buffer[0..need])?;
                if self.length <= 0 {
                    break;
                }
                self.position = 0;
                self.next -= self.length;
            }
            while (dst_offset < buffer.len()) && (self.position < self.length) {
                let mut src_size = (self.length - self.position) as size_t;
                let mut dst_size = (buffer.len() - dst_offset) as size_t;
                let length = wrap_error(unsafe {
                    LZ4F_decompress(
                        self.context.0,
                        buffer[dst_offset..].as_mut_ptr(),
                        &mut dst_size,
                        self.buffer[self.position..].as_ptr(),
                        &mut src_size,
                        ptr::null(),
                    )
                })?;
                self.position += src_size as usize;
                dst_offset += dst_size as usize;
                if length == 0 {
                    self.next = 0;
                    return Ok(dst_offset);
                } else if self.next < length {
                    self.next = length;
                }
            }
        }
        Ok(dst_offset)
    }
}
