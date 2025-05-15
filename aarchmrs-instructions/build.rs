/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-changed=");
    if env::var("FORCE_AARCHMRS_GEN").is_ok() {
        let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let cache_dir = env::var("OUT_DIR").unwrap();
        aarchmrs_gen::gen_instructions(
            &PathBuf::from(out_dir).join("src"),
            &PathBuf::from(cache_dir),
        )
        .unwrap();
    }
}
