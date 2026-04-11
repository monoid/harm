/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

#[cfg(feature = "memmap2")]
mod memmap2;

#[cfg(feature = "memmap2")]
pub use self::memmap2::{Mmap2Buffer, Mmap2FixedMemory};

pub trait Memory<FM: FixedMemory> {
    type ExtendError;
    type FixedMemoryError;

    /// Current writing position.
    fn pos(&self) -> usize;

    /// If memory has fixed capacity, return it.
    ///
    /// A `Vec` is not considered a memory of fixed capacity because it can grow indefinitely.
    fn capacity(&self) -> Option<usize>;

    /// Append data to the memory.  Should fail when it reaches memory's capacity.
    fn try_extend<I: Iterator<Item = u8>>(&mut self, bytes: I) -> Result<(), Self::ExtendError>;

    /// Transform into fixed-location memory.
    fn into_fixed_memory(self) -> Result<FM, Self::FixedMemoryError>;

    /// Align position.
    fn align(&mut self, alignment: usize) -> Result<(), Self::ExtendError> {
        let pos = self.pos();
        let remn = pos % alignment;
        if remn != 0 {
            self.try_extend(core::iter::repeat(0).take(alignment - remn))?;
        }
        Ok(())
    }
}

/// Memory with fixed location that can be transformed to an executable one after relocations are applied.
pub trait FixedMemory: AsMut<[u8]> {
    type ExecutableMemory;
    type ExecutableMemoryError;

    fn into_executable_memory(self) -> Result<Self::ExecutableMemory, Self::ExecutableMemoryError>;
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "memmap2")]
    #[test]
    fn test_align() {
        use super::*;

        let mut data = Vec::<u8>::new();

        Memory::align(&mut data, 8);
        assert!(data.is_empty());

        data.push(1);
        Memory::align(&mut data, 8);
        assert_eq!(data.len(), 8);

        Memory::align(&mut data, 8);
        assert_eq!(data.len(), 8);

        data.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7]);
        Memory::align(&mut data, 8);
        assert_eq!(data.len(), 16);
    }
}
