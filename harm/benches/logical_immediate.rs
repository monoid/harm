/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::{collections::HashMap, hint::black_box};

use criterion::*;
use harm::instructions::dpimm::log_imm::immediate::{
    LogicalImmFields, PackedLogicalImm, decode_logical_immediate, encode_logical_immediate64,
};
use ptr_hash::{PtrHash, PtrHashParams};
use rand::{Rng, seq::SliceRandom};

fn bench_encoding_64_logical_immediate(c: &mut Criterion) {
    // gen data: correct and incorrect
    let correct_i64: HashMap<_, _> = (0..(1 << 13))
        .filter_map(move |n| {
            let g = PackedLogicalImm::new(n).expect("value in range");
            let fields = LogicalImmFields::from_packed(g);

            decode_logical_immediate(fields, true, true).map(|key| (key, fields))
        })
        .collect();

    let mut rng = rand::rng();
    // on the same data:
    let mut random_data: Vec<u64> = (0..correct_i64.len())
        .map(|_| rng.random::<u64>())
        .collect();
    random_data.extend(correct_i64.keys().copied());
    random_data.shuffle(&mut rng);

    // optimized
    c.bench_function("encode_logical_immediate", |b| {
        b.iter(|| {
            for &v in &random_data {
                black_box(encode_logical_immediate64(v));
            }
        });
    });

    // hash table
    c.bench_function("hash_table", |b| {
        b.iter_batched(
            || correct_i64.clone(),
            |ht| {
                for &v in &random_data {
                    black_box(ht.get(&v).copied());
                }
            },
            BatchSize::SmallInput,
        );
    });

    // ptr_hash
    let pairs: Vec<_> = correct_i64.iter().map(|(&key, &val)| (key, val)).collect();
    let keys: Vec<_> = pairs.iter().map(|&(key, _)| key).collect();

    let hash = <PtrHash>::new(&keys, PtrHashParams::default());

    // On M1, it is barely behind encode_logical_immediate.  When `pairs` is not copied (i.e. it is
    // in the cache) it is faster.
    //
    // However, there seems to be no convenient way to define PtrHash in compile-time.  A lazy
    // static may easily make everything slower.
    c.bench_function("ptr_hash", |b| {
        b.iter_batched(
            || (hash.clone(), pairs.clone()),
            |(ht, pairs)| {
                for &v in &random_data {
                    let idx = ht.index(&v);
                    black_box(if pairs[idx].0 == v {
                        Some(pairs[idx].1)
                    } else {
                        None
                    });
                }
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(logical_immediate, bench_encoding_64_logical_immediate);
criterion_main!(logical_immediate);
