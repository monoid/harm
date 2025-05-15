use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-changed=");
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    aarchmrs_gen::gen_data(&PathBuf::from(out_dir).join("src")).unwrap();
}
