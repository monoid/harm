/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This file is licensed under the BSD 3-clause license.
 */
use clap::Parser;
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    r#mod: bool,
    #[clap(long)]
    temp_dir: Option<PathBuf>,
    src_dest_dir: PathBuf,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let temp_dir = args.temp_dir.unwrap_or_else(env::temp_dir);
    aarchmrs_gen::gen_instructions(&args.src_dest_dir, &temp_dir, args.r#mod).unwrap();

    eprintln!("* Please, run `cargo fmt` in the respective crate.");
    Ok(())
}
