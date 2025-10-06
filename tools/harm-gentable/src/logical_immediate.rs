/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::collections::HashMap;

use harm::bits::UBitValue;

pub type PackedLogicalImm = UBitValue<13>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LogicalImmFields {
    n: UBitValue<1>,
    immr: UBitValue<6>,
    imms: UBitValue<6>,
}

impl LogicalImmFields {
    pub fn from_packed(g: PackedLogicalImm) -> Self {
        let n = UBitValue::new(g.bits() >> 12).unwrap();
        let immr = UBitValue::new((g.bits() >> 6) & 0b111111).unwrap();
        let imms = UBitValue::new(g.bits() & 0b111111).unwrap();
        Self { n, immr, imms }
    }
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
        pattern = pattern & 0xFFFFFFFF;
    }

    return Some(pattern);
}

pub fn gen_values(is_64_bits: bool) -> impl Iterator<Item = (u64, (u8, u8, u8))> {
    (0..(1 << 13)).filter_map(move |n| {
        let g = PackedLogicalImm::new(n).expect("value in range");
        let fields = LogicalImmFields::from_packed(g);

        decode_logical_immediate(fields, true, is_64_bits).map(|key| {
            (
                key,
                (
                    fields.n.bits() as u8,
                    fields.immr.bits() as _,
                    fields.imms.bits() as _,
                ),
            )
        })
    })
}

pub fn print_values(is_64_bits: bool) {
    let mut hashmap = HashMap::new();
    for (key, val) in gen_values(is_64_bits) {
        hashmap.entry(key).or_insert(val);
    }

    let mut items: Vec<_> = hashmap.into_iter().collect();
    items.sort_unstable_by_key(|p| p.0);
    for (key, (n, immr, imms)) in items {
        if is_64_bits {
            println!("0x{key:016x} => ({n}, 0b{immr:06b}, 0b{imms:06b})");
        } else {
            assert!(n == 0);
            assert!(u32::try_from(key).is_ok(), "{key}");
            println!("0x{key:08x} => ({n}, 0b{immr:06b}, 0b{imms:06b})");
        }
    }
}
