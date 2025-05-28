/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(align(4))]
pub struct InstructionCode(pub [u8; 4]);

impl InstructionCode {
    pub const fn from_u32(value: u32) -> Self {
        // B2.6.2 Instruction endianness
        // A64 instructions have a fixed length of 32 bits and are always little-endian.
        Self(value.to_le_bytes())
    }

    pub const fn unpack(self) -> u32 {
        u32::from_be_bytes(self.0)
    }
}
