use crate::{context::*, sys::*};

pub use std::io::Write;
use std::{
    cmp::min,
    io::{copy, Read, Result as IOResult},
    ptr,
};

use libc::size_t;

/// Configuration builder for the `Encoder` structure.
#[derive(Clone, Copy)]
pub struct EncoderBuilder {
    block_size:      BlockSize,
    block_mode:      BlockMode,
    checksum:        ContentChecksum,
    level:           u32,
    auto_flush:      bool,
    favor_dec_speed: bool,
}

/// Encoder for LZ4 frame format data.
pub struct Encoder<W: Write> {
    context: LZ4FCompressionContext,
    writer:  Option<W>,
    limit:   usize,
    buffer:  Vec<u8>,
}

impl Default for EncoderBuilder {
    fn default() -> Self {
        Self {
            block_size:      BlockSize::Max64KB,
            block_mode:      BlockMode::Linked,
            checksum:        ContentChecksum::ChecksumEnabled,
            level:           0,
            auto_flush:      false,
            favor_dec_speed: false,
        }
    }
}

impl EncoderBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn block_size(&mut self, block_size: BlockSize) -> &mut Self {
        self.block_size = block_size;
        self
    }

    pub fn block_mode(&mut self, block_mode: BlockMode) -> &mut Self {
        self.block_mode = block_mode;
        self
    }

    pub fn checksum(&mut self, checksum: ContentChecksum) -> &mut Self {
        self.checksum = checksum;
        self
    }

    pub fn level(&mut self, level: u32) -> &mut Self {
        self.level = level;
        self
    }

    pub fn auto_flush(&mut self, auto_flush: bool) -> &mut Self {
        self.auto_flush = auto_flush;
        self
    }

    pub fn favor_dec_speed(&mut self, favor_dec_speed: bool) -> &mut Self {
        self.favor_dec_speed = favor_dec_speed;
        self
    }

    pub fn build<W: Write>(&self, writer: W) -> IOResult<Encoder<W>> {
        let preferences = LZ4FPreferences {
            frame_info:        LZ4FFrameInfo {
                block_size_id:         self.block_size,
                block_mode:            self.block_mode,
                content_checksum_flag: self.checksum,
                frame_type:            LZ4FrameType::LZ4Frame,
                content_size:          0,
                dict_id:               0,
                block_checksum_flag:   BlockChecksum::NoChecksum,
            },
            compression_level: self.level,
            auto_flush:        self.auto_flush as u32,
            favor_dec_speed:   self.favor_dec_speed as u32,
            reserved:          [0; 3],
        };

        let mut encoder = Encoder {
            context: LZ4FCompressionContext::new()?,
            writer:  Some(writer),
            limit:   self.block_size.bytes(),
            buffer:  Vec::with_capacity(wrap_error(unsafe {
                LZ4F_compressBound(self.block_size.bytes() as size_t, &preferences)
            })?),
        };
        encoder.write_header(&preferences)?;
        Ok(encoder)
    }

    pub fn encode<R: Read>(&self, reader: &mut R) -> IOResult<Vec<u8>> {
        let mut encoder = self.build(Vec::new())?;
        copy(reader, &mut encoder)?;
        encoder.finish()
    }
}

/// Trait for encoding `Read` implementing objects with an `EncoderBuilder`.
pub trait Encode {
    fn encode(&mut self, builder: &EncoderBuilder) -> IOResult<Vec<u8>>;
}

impl<R> Encode for R
where
    R: Read,
{
    fn encode(&mut self, builder: &EncoderBuilder) -> IOResult<Vec<u8>> { builder.encode(self) }
}

impl<W: Write> Encoder<W> {
    fn write_header(&mut self, preferences: &LZ4FPreferences) -> IOResult<()> {
        unsafe {
            let len = wrap_error(LZ4F_compressBegin(
                self.context.0,
                self.buffer.as_mut_ptr(),
                self.buffer.capacity() as size_t,
                preferences,
            ))?;
            self.buffer.set_len(len);
        }
        self.writer.as_mut().unwrap().write_all(&self.buffer)
    }

    fn write_end(&mut self) -> IOResult<()> {
        unsafe {
            let len = wrap_error(LZ4F_compressEnd(
                self.context.0,
                self.buffer.as_mut_ptr(),
                self.buffer.capacity() as size_t,
                ptr::null(),
            ))?;
            self.buffer.set_len(len);
        };
        self.writer.as_mut().unwrap().write_all(&self.buffer)
    }

    pub fn finish(mut self) -> IOResult<W> { self.write_end().map(|_| self.writer.take().unwrap()) }
}

impl<W: Write> Drop for Encoder<W> {
    fn drop(&mut self) {
        if self.writer.is_some() {
            let _ = self.write_end();
        }
    }
}

impl<W: Write> Write for Encoder<W> {
    fn write(&mut self, buffer: &[u8]) -> IOResult<usize> {
        let mut offset = 0;
        while offset < buffer.len() {
            let size = min(buffer.len() - offset, self.limit);
            unsafe {
                let len = wrap_error(LZ4F_compressUpdate(
                    self.context.0,
                    self.buffer.as_mut_ptr(),
                    self.buffer.capacity() as size_t,
                    buffer[offset..].as_ptr(),
                    size as size_t,
                    ptr::null(),
                ))?;
                self.buffer.set_len(len);
                self.writer.as_mut().unwrap().write_all(&self.buffer)?;
            }
            offset += size;
        }
        Ok(buffer.len())
    }

    fn flush(&mut self) -> IOResult<()> {
        loop {
            unsafe {
                let len = wrap_error(LZ4F_flush(
                    self.context.0,
                    self.buffer.as_mut_ptr(),
                    self.buffer.capacity() as size_t,
                    ptr::null(),
                ))?;
                if len == 0 {
                    break;
                }
                self.buffer.set_len(len);
            };
            self.writer.as_mut().unwrap().write_all(&self.buffer)?;
        }
        self.writer.as_mut().unwrap().flush()
    }
}
