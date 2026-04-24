/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::convert::Infallible;

use harm::reloc::Addr64;

use super::{IntoPositionedMemory, Memory, PositionedMemory};

#[derive(thiserror::Error, Debug)]
pub enum MapBufferError {
    #[error("buffer overflow: {0}")]
    Overflow(usize),
}

pub struct MmapBuffer {
    pos: usize,
    memory: memmap2::MmapMut,
}

impl MmapBuffer {
    #[inline]
    pub fn new(mmap_mut: memmap2::MmapMut) -> Self {
        Self {
            pos: 0,
            // N.B. We assume that memory is aligned.
            memory: mmap_mut,
        }
    }

    #[inline]
    pub fn allocate(length: usize) -> std::io::Result<Self> {
        let mmap_mut = memmap2::MmapMut::map_anon(length)?;
        Ok(Self::new(mmap_mut))
    }
}

impl Memory for MmapBuffer {
    type ExtendError = MapBufferError;

    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    fn capacity(&self) -> Option<usize> {
        Some(self.memory.len())
    }

    #[inline]
    fn try_extend<I: Iterator<Item = u8>>(&mut self, bytes: I) -> Result<(), Self::ExtendError> {
        for byte in bytes {
            if self.pos >= self.memory.len() {
                return Err(MapBufferError::Overflow(self.pos));
            }

            self.memory[self.pos] = byte;
            self.pos += 1;
        }
        Ok(())
    }
}

impl IntoPositionedMemory<MmapPositionedMemory> for MmapBuffer {
    type PositionedMemoryError = Infallible;

    #[inline]
    fn into_positioned_memory(self) -> Result<MmapPositionedMemory, Self::PositionedMemoryError> {
        Ok(MmapPositionedMemory::new(self.memory))
    }
    
}

pub struct MmapPositionedMemory(memmap2::MmapMut);

impl MmapPositionedMemory {
    #[inline]
    pub fn new(mmap_mut: memmap2::MmapMut) -> Self {
        Self(mmap_mut)
    }

    #[inline]
    pub fn allocate(length: usize) -> std::io::Result<Self> {
        let mmap_mut = memmap2::MmapMut::map_anon(length)?;
        Ok(Self(mmap_mut))
    }
}

impl AsRef<[u8]> for MmapPositionedMemory {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for MmapPositionedMemory {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl PositionedMemory for MmapPositionedMemory {
    // TODO a wrapper type?
    type ExecutableMemory = memmap2::Mmap;

    type ExecutableMemoryError = std::io::Error;

    // TODO makes sense only on AArch64.
    fn get_base_address(&self) -> Addr64 {
        self.0.as_ptr() as Addr64
    }

    #[inline]
    fn into_executable_memory(self) -> Result<Self::ExecutableMemory, Self::ExecutableMemoryError> {
        self.0.make_exec()
    }
}

impl Memory for &mut Vec<u8> {
    type ExtendError = Infallible;

    #[inline]
    fn pos(&self) -> usize {
        self.len()
    }

    #[inline]
    fn capacity(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn try_extend<I: Iterator<Item = u8>>(&mut self, bytes: I) -> Result<(), Self::ExtendError> {
        self.extend(bytes);
        Ok(())
    }
}

impl IntoPositionedMemory<MmapPositionedMemory> for &mut Vec<u8> {
    type PositionedMemoryError = std::io::Error;

    #[inline]
    fn into_positioned_memory(self) -> Result<MmapPositionedMemory, Self::PositionedMemoryError> {
        let mut mem = MmapPositionedMemory::allocate(self.len())?;
        // The memmap2 spec doesn't say that the length can be different...
        mem.as_mut().copy_from_slice(self);
        Ok(mem)
    }
}

#[cfg(test)]
mod tests {
    use harm::instructions::InstructionSeq;

    use super::*;

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_mmap_execute() {
        use harm::{
            instructions::{arith::add::add, control::ret},
            register::Reg64::*,
        };
        let mut buf = MmapBuffer::allocate(8).expect("mmap failed, system problem");
        buf.try_extend(add(X0, X0, X1).bytes()).unwrap();
        buf.try_extend(ret().bytes()).unwrap();

        let mem = buf.into_positioned_memory().unwrap();
        // Doing relocations...

        let exec = mem.into_executable_memory().unwrap();

        let res;
        unsafe {
            clear_cache::clear_cache(exec.as_ptr(), exec.as_ptr().add(exec.len()));

            let func: unsafe extern "C" fn(i64, i64) -> i64 = std::mem::transmute(exec.as_ptr());
            res = func(1, 2);
        }
        assert_eq!(res, 3);
    }

    #[test]
    fn test_try_extend_1023() {
        let mut buf = MmapBuffer::allocate(1024).expect("mmap failed, system problem");
        buf.try_extend(vec![1; 1023].into_iter()).unwrap();
    }

    #[test]
    fn test_try_extend_1024() {
        let mut buf = MmapBuffer::allocate(1024).expect("mmap failed, system problem");
        buf.try_extend(vec![1; 1024].into_iter()).unwrap();
    }

    #[test]
    fn test_try_extend_1025() {
        let mut buf = MmapBuffer::allocate(1024).expect("mmap failed, system problem");
        assert!(buf.try_extend(vec![1; 1025].into_iter()).is_err());
    }

    #[test]
    fn test_try_extend_pair() {
        let mut buf = MmapBuffer::allocate(1024).expect("mmap failed, system problem");
        buf.try_extend(vec![1; 512].into_iter()).unwrap();
        buf.try_extend(vec![1; 512].into_iter()).unwrap();
        assert_eq!(buf.pos(), 1024);

        assert!(buf.try_extend(vec![1; 1].into_iter()).is_err());
    }
}
