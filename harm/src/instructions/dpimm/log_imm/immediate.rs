/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::bits::UBitValue;

pub type PackedLogicalImm = UBitValue<13>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LogicalImmFields {
    pub n: UBitValue<1>,
    pub immr: UBitValue<6>,
    pub imms: UBitValue<6>,
}

impl LogicalImmFields {
    pub fn try_new64(val: u64) -> Result<Self, LogicalImmError> {
        encode_logical_immediate64(val).ok_or(LogicalImmError::InvalidLogicalImmediate)
    }

    pub fn try_new32(val: u32) -> Result<Self, LogicalImmError> {
        encode_logical_immediate32(val).ok_or(LogicalImmError::InvalidLogicalImmediate)
    }

    pub fn from_packed(g: PackedLogicalImm) -> Self {
        let n = UBitValue::new(g.bits() >> 12).unwrap();
        let immr = UBitValue::new((g.bits() >> 6) & 0b111111).unwrap();
        let imms = UBitValue::new(g.bits() & 0b111111).unwrap();
        Self { n, immr, imms }
    }
}

#[derive(Debug)]
pub enum LogicalImmError {
    InvalidLogicalImmediate,
}

use core::fmt;
impl fmt::Display for LogicalImmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicalImmError::InvalidLogicalImmediate => write!(f, "invalid logical immediate"),
        }
    }
}

impl core::error::Error for LogicalImmError {}

// This code is based on public domain code by Dougall Johnson published at
// <https://gist.github.com/dougallj/97d8621d4542ba31e004acc8075fac14>. See also a companion article at
// https://dougallj.wordpress.com/2021/10/30/bit-twiddling-optimising-aarch64-logical-immediate-encoding-and-decoding/
pub fn encode_logical_immediate64(val: u64) -> Option<LogicalImmFields> {
    if val == 0 || !val == 0 {
        return None;
    }

    let rotation = clear_trailing_ones(val).trailing_zeros();
    let normalized = val.rotate_right(rotation & 63);
    let zeroes = normalized.leading_zeros();
    let ones = normalized.trailing_ones();
    let size = zeroes + ones;
    if val.rotate_right(size & 63) != val {
        return None;
    }
    let immr = rotation.wrapping_neg() & (size - 1);
    let imms = ((size << 1).wrapping_neg() | (ones - 1)) & 0x3F;
    let n = size >> 6;

    // The current compiler cannot proof that n and immr fit into the range, and generate code for "possible" panic.
    // Mask it explicitly to avoid check and never-executing branch.
    Some(LogicalImmFields {
        n: UBitValue::new(n & 1).unwrap(),
        immr: UBitValue::new(immr & 0x3F).unwrap(),
        imms: UBitValue::new(imms).unwrap(),
    })
}

pub fn encode_logical_immediate32(val: u32) -> Option<LogicalImmFields> {
    let val: u64 = val.into();
    encode_logical_immediate64((val << 32) | val).inspect(|fields| {
        debug_assert!(fields.n.bits() == 0);
    })
}

// Mindlessly follows DecodeBitMask from AArch64 spec, though a better implementation exists.
pub fn decode_logical_immediate(
    fields: LogicalImmFields,
    immediate: bool,
    is_64_bits: bool,
) -> Option<u64> {
    let m: u32 = if is_64_bits { 64 } else { 32 };

    // Compute log2 of element size
    // 2^len must be in range [2, M]
    let len =
        31i32 - ((fields.n.bits() << 6) | (!fields.imms.bits()) & 0x3F).leading_zeros() as i32;
    if len < 1 {
        return None;
    }
    if m < (1u32 << len) {
        return None;
    }

    let levels = (1 << len) - 1;
    // For logical immediates an all-ones value of S is reserved
    // since it would generate a useless all-ones result (many times)
    if immediate && (fields.imms.bits() & levels) == levels {
        return None;
    }

    let s = fields.imms.bits() & levels;
    let r = fields.immr.bits() & levels;

    let mut size = 1u32 << len;
    let mut pattern = (1u64 << (s + 1)) - 1;

    while size <= u64::BITS {
        pattern |= pattern.wrapping_shl(size);
        size *= 2;
    }

    pattern = pattern.rotate_right(r);
    if !is_64_bits {
        pattern &= 0xFFFFFFFF;
    }

    Some(pattern)
}

fn clear_trailing_ones(n: u64) -> u64 {
    n & (n + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_64_fffe000000000007() {
        let f = encode_logical_immediate64(0xfffe000000000007);
        assert_eq!(
            f,
            Some(LogicalImmFields {
                n: 1.try_into().unwrap(),
                immr: 0b001111.try_into().unwrap(),
                imms: 0b010001.try_into().unwrap(),
            })
        );
    }

    #[test]
    fn test_64_00000000003fffe0() {
        let f = encode_logical_immediate64(0x00000000003fffe0);
        assert_eq!(
            f,
            Some(LogicalImmFields {
                n: 1.try_into().unwrap(),
                immr: 0b111011.try_into().unwrap(),
                imms: 0b010000.try_into().unwrap(),
            })
        );
    }

    #[test]
    fn test_64_fc3ffffffc3fffff() {
        let f = encode_logical_immediate64(0xfc3ffffffc3fffff);
        assert_eq!(
            f,
            Some(LogicalImmFields {
                n: 0.try_into().unwrap(),
                immr: 0b000110.try_into().unwrap(),
                imms: 0b011011.try_into().unwrap(),
            })
        );
    }

    #[test]
    fn test_64_0xffffe7ffffffe7ff() {
        let f = encode_logical_immediate64(0xffffe7ffffffe7ff);
        assert_eq!(
            f,
            Some(LogicalImmFields {
                n: 0.try_into().unwrap(),
                immr: 0b010011.try_into().unwrap(),
                imms: 0b011101.try_into().unwrap(),
            })
        );
    }

    #[test]
    fn test_64_failing() {
        assert!(encode_logical_immediate64(0).is_none());
        assert!(encode_logical_immediate64(u64::MAX).is_none());
        assert!(encode_logical_immediate64(42).is_none());
        assert!(encode_logical_immediate64(0x0101010301010101).is_none());
    }

    #[test]
    fn test_32_e000007f() {
        let f = encode_logical_immediate32(0xe000007f);
        assert_eq!(
            f,
            Some(LogicalImmFields {
                n: 0.try_into().unwrap(),
                immr: 0b000011.try_into().unwrap(),
                imms: 0b001001.try_into().unwrap(),
            })
        );
    }

    #[test]
    fn test_32_ffffc3ff() {
        let f = encode_logical_immediate32(0xffffc3ff);
        assert_eq!(
            f,
            Some(LogicalImmFields {
                n: 0.try_into().unwrap(),
                immr: 0b010010.try_into().unwrap(),
                imms: 0b011011.try_into().unwrap(),
            })
        );
    }

    #[test]
    fn test_32_f000f000() {
        let f = encode_logical_immediate32(0xf000f000);
        assert_eq!(
            f,
            Some(LogicalImmFields {
                n: 0.try_into().unwrap(),
                immr: 0b000100.try_into().unwrap(),
                imms: 0b100011.try_into().unwrap(),
            })
        );
    }

    #[test]
    fn test_32_1f800000() {
        let f = encode_logical_immediate32(0x1f800000);
        assert_eq!(
            f,
            Some(LogicalImmFields {
                n: 0.try_into().unwrap(),
                immr: 0b001001.try_into().unwrap(),
                imms: 0b000101.try_into().unwrap(),
            })
        );
    }

    #[test]
    fn test_32_failing() {
        assert!(encode_logical_immediate32(0).is_none());
        assert!(encode_logical_immediate32(u32::MAX).is_none());
        assert!(encode_logical_immediate32(42).is_none());
        assert!(encode_logical_immediate32(0x03010101).is_none());
    }
}
