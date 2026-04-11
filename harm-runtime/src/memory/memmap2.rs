/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::convert::Infallible;

use super::{FixedMemory, Memory};

#[derive(thiserror::Error, Debug)]
pub enum Map2BufferError {
    #[error("buffer overflow: {0}")]
    Overflow(usize),
}

pub struct Mmap2Buffer {
    pos: usize,
    memory: memmap2::MmapMut,
}

impl Mmap2Buffer {
    #[inline]
    pub fn new(mmap_mut: memmap2::MmapMut) -> Self {
        Self {
            pos: 0,
            memory: mmap_mut,
        }
    }

    #[inline]
    pub fn allocate(length: usize) -> std::io::Result<Self> {
        let mmap_mut = memmap2::MmapMut::map_anon(length)?;
        Ok(Self::new(mmap_mut))
    }
}

impl Memory<Mmap2FixedMemory> for Mmap2Buffer {
    type ExtendError = Map2BufferError;

    type FixedMemoryError = Infallible;

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
                return Err(Map2BufferError::Overflow(self.pos));
            }

            self.memory[self.pos] = byte;
            self.pos += 1;
        }
        Ok(())
    }

    #[inline]
    fn into_fixed_memory(self) -> Result<Mmap2FixedMemory, Self::FixedMemoryError> {
        Ok(Mmap2FixedMemory::new(self.memory))
    }
}

pub struct Mmap2FixedMemory(memmap2::MmapMut);

impl Mmap2FixedMemory {
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

impl AsRef<[u8]> for Mmap2FixedMemory {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Mmap2FixedMemory {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl FixedMemory for Mmap2FixedMemory {
    // TODO a wrapper type?
    type ExecutableMemory = memmap2::Mmap;

    type ExecutableMemoryError = std::io::Error;

    #[inline]
    fn into_executable_memory(self) -> Result<Self::ExecutableMemory, Self::ExecutableMemoryError> {
        self.0.make_exec()
    }
}

impl Memory<Mmap2FixedMemory> for Vec<u8> {
    type ExtendError = Infallible;
    type FixedMemoryError = std::io::Error;

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

    #[inline]
    fn into_fixed_memory(self) -> Result<Mmap2FixedMemory, Self::FixedMemoryError> {
        let mut mem = Mmap2FixedMemory::allocate(self.len())?;
        mem.as_mut().copy_from_slice(&self);
        Ok(mem)
    }
}

impl Memory<Mmap2FixedMemory> for &mut Vec<u8> {
    type ExtendError = Infallible;
    type FixedMemoryError = std::io::Error;

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

    #[inline]
    fn into_fixed_memory(self) -> Result<Mmap2FixedMemory, Self::FixedMemoryError> {
        let mut mem = Mmap2FixedMemory::allocate(self.len())?;
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
    fn test_mmap() {
        let mut buf = Mmap2Buffer::allocate(1024).expect("mmap failed, system problem");
        buf.try_extend(harm::instructions::control::ret().bytes())
            .unwrap();

        let mem = buf.into_fixed_memory().unwrap();
        // Doing relocations...

        let exec = mem.into_executable_memory().unwrap();

        unsafe {
            let func: unsafe extern "C" fn() = std::mem::transmute(exec.as_ptr());
            func();
        }
    }

    #[test]
    fn test_try_extend_1023() {
        let mut buf = Mmap2Buffer::allocate(1024).expect("mmap failed, system problem");
        buf.try_extend(vec![1; 1023].into_iter()).unwrap();
    }

    #[test]
    fn test_try_extend_1024() {
        let mut buf = Mmap2Buffer::allocate(1024).expect("mmap failed, system problem");
        buf.try_extend(vec![1; 1024].into_iter()).unwrap();
    }

    #[test]
    fn test_try_extend_1025() {
        let mut buf = Mmap2Buffer::allocate(1024).expect("mmap failed, system problem");
        assert!(buf.try_extend(vec![1; 1025].into_iter()).is_err());
    }

    #[test]
    fn test_try_extend_pair() {
        let mut buf = Mmap2Buffer::allocate(1024).expect("mmap failed, system problem");
        buf.try_extend(vec![1; 512].into_iter()).unwrap();
        buf.try_extend(vec![1; 512].into_iter()).unwrap();
        assert_eq!(buf.pos(), 1024);

        assert!(buf.try_extend(vec![1; 1].into_iter()).is_err());
    }
}
