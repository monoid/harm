/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::collections::HashMap;

use harm::instructions::dpimm::log_imm::immediate::{
    LogicalImmFields, PackedLogicalImm, decode_logical_immediate,
};

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
