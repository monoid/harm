/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

mod logical_immediate;

use clap::Parser;
use logical_immediate::print_values;

#[derive(Debug, clap::Subcommand)]
enum Command {
    LogicalImmediate32,
    LogicalImmediate64,
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::LogicalImmediate32 => print_values(false),
        Command::LogicalImmediate64 => print_values(true),
    }
}
