/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use harm::reloc::Addr;

use super::Memory;

/// Memory that is not intended to be executed immediately, but stored or transferred.
pub struct ForeignMemoryBuffer {
    mem: alloc::vec::Vec<u8>,
    base_addr: Addr,
}

impl<'mem> ForeignMemoryBuffer {
    pub fn new(base_addr: Addr) -> Self {
        Self {
            mem: Vec::new(),
            base_addr,
        }
    }

    pub fn with_capacity(base_addr: Addr, capacity: usize) -> Self {
        Self {
            mem: Vec::with_capacity(capacity),
            base_addr,
        }
    }

    pub fn base_addr(&self) -> Addr {
        self.base_addr
    }
}

impl AsRef<[u8]> for ForeignMemoryBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.mem
    }
}

impl Memory<ForeignMemory> for ForeignMemoryBuffer {
    type ExtendError = core::convert::Infallible;
    type FixedMemoryError = core::convert::Infallible;

    fn pos(&self) -> usize {
        self.mem.len()
    }

    fn capacity(&self) -> Option<usize> {
        None // unrestricted
    }

    fn try_extend<I: Iterator<Item = u8>>(&mut self, bytes: I) -> Result<(), Self::ExtendError> {
        self.mem.extend(bytes);
        Ok(())
    }

    fn into_fixed_memory(self) -> Result<ForeignMemory, Self::FixedMemoryError> {
        Ok(ForeignMemory {
            mem: self.mem,
            base_addr: self.base_addr,
        })
    }
}

pub struct ForeignMemory {
    mem: alloc::vec::Vec<u8>,
    base_addr: Addr,
}

impl ForeignMemory {
    pub fn base_addr(&self) -> Addr {
        self.base_addr
    }
    pub fn into_inner(self) -> (Addr, alloc::vec::Vec<u8>) {
        (self.base_addr, self.mem)
    }
}

impl AsRef<[u8]> for ForeignMemory {
    fn as_ref(&self) -> &[u8] {
        &self.mem
    }
}

impl AsMut<[u8]> for ForeignMemory {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.mem
    }
}
