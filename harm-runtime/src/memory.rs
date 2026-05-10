/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

#[cfg(feature = "memmap2")]
mod memmap2;

use harm::reloc::Addr64;

#[cfg(feature = "memmap2")]
pub use self::memmap2::{MmapBuffer, MmapPositionedMemory};

pub mod foreign_memory;
pub use self::foreign_memory::ForeignMemoryBuffer;

pub trait Memory {
    type ExtendError;

    /// Current writing position.
    fn pos(&self) -> usize;

    /// If the memory has fixed capacity, return it.
    ///
    /// A `Vec` is not considered a memory of fixed capacity because it can grow indefinitely.
    fn capacity(&self) -> Option<usize>;

    /// Append data to the memory.
    ///
    /// Should fail when it reaches memory's capacity. In this case, `self.pos()` must not change, but the memory behind
    /// it till the end of the capacity may be modified.
    fn try_extend<I: Iterator<Item = u8>>(&mut self, bytes: I) -> Result<(), Self::ExtendError>;

    /// Align position.  Same guarantees as `try_extend` apply.
    fn align(&mut self, alignment: usize) -> Result<(), Self::ExtendError> {
        if alignment > 1 {
            let pos = self.pos();
            let remn = pos % alignment;
            if remn != 0 {
                self.try_extend(core::iter::repeat_n(0, alignment - remn))?;
            }
        }
        Ok(())
    }
}

pub trait IntoPositionedMemory<PM> {
    type PositionedMemoryError;

    /// Transform into positioned memory.
    fn into_positioned_memory(self) -> Result<PM, Self::PositionedMemoryError>;
}

/// Memory with fixed location that can be transformed to an executable one after relocations are applied.
pub trait PositionedMemory: AsMut<[u8]> {
    fn get_base_address(&self) -> Addr64;
}

pub trait IntoExecutableMemory {
    type ExecutableMemory;
    type ExecutableMemoryError;

    fn into_executable_memory(self) -> Result<Self::ExecutableMemory, Self::ExecutableMemoryError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_align() {
        use super::*;

        let mut data = &mut Vec::<u8>::new();

        Memory::align(&mut data, 8).unwrap();
        assert!(data.is_empty());

        data.push(1);
        Memory::align(&mut data, 8).unwrap();
        assert_eq!(data.len(), 8);

        Memory::align(&mut data, 8).unwrap();
        assert_eq!(data.len(), 8);

        data.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7]);
        Memory::align(&mut data, 8).unwrap();
        assert_eq!(data.len(), 16);
    }

    #[test]
    fn test_align_corner_case() {
        use super::*;

        let mut data = &mut Vec::<u8>::new();

        Memory::align(&mut data, 0).unwrap();
        assert!(data.is_empty());

        data.push(1);
        Memory::align(&mut data, 0).unwrap();
        assert_eq!(data.len(), 1);

        Memory::align(&mut data, 1).unwrap();
        assert_eq!(data.len(), 1);
    }
}
