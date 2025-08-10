/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use hex::{FromHex, ToHex};

/// ARM64 instruction with proper byte order and alignment.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(align(4))]
pub struct InstructionCode(pub [u8; 4]);

impl InstructionCode {
    #[inline]
    pub const fn from_u32(value: u32) -> Self {
        // B2.6.2 Instruction endianness
        // A64 instructions have a fixed length of 32 bits and are always little-endian.
        Self(value.to_le_bytes())
    }

    #[inline]
    pub const fn unpack(self) -> u32 {
        u32::from_le_bytes(self.0)
    }
}

impl FromHex for InstructionCode {
    type Error = <[u8; 4] as FromHex>::Error;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        <[u8; 4]>::from_hex(hex).map(|bytes| Self::from_u32(u32::from_be_bytes(bytes)))
    }
}

impl ToHex for InstructionCode {
    fn encode_hex<T: core::iter::FromIterator<char>>(&self) -> T {
        <[u8; 4]>::encode_hex(&self.unpack().to_be_bytes())
    }

    fn encode_hex_upper<T: core::iter::FromIterator<char>>(&self) -> T {
        <[u8; 4]>::encode_hex_upper(&self.unpack().to_be_bytes())
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use alloc::string::String;

    #[test]
    fn test_from_u32() {
        assert_eq!(
            InstructionCode::from_u32(0x12_f3_56_ab),
            InstructionCode([0xab, 0x56, 0xf3, 0x12]),
        );
    }

    #[test]
    fn test_unpack() {
        assert_eq!(
            InstructionCode([0xab, 0x56, 0xf3, 0x12]).unpack(),
            0x12_f3_56_ab,
        );
    }

    #[test]
    fn test_fromhex() {
        assert_eq!(
            InstructionCode::from_hex("12f356ab"),
            Ok(InstructionCode([0xab, 0x56, 0xf3, 0x12])),
        );
    }

    #[test]
    fn test_encode_hex() {
        assert_eq!(
            InstructionCode([0xab, 0x56, 0xf3, 0x12]).encode_hex::<String>(),
            "12f356ab",
        );
    }

    #[test]
    fn test_encode_hex_upper() {
        assert_eq!(
            InstructionCode([0xab, 0x56, 0xf3, 0x12]).encode_hex_upper::<String>(),
            "12F356AB",
        );
    }
}
