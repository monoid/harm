/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::io;
use std::path::Path;

use aarchmrs_parser::instructions::{
    Encodeset, InstructionGroup, InstructionGroupOrInstruction, License,
};
use proc_macro2::TokenStream;

use crate::downloads::{DownloadError, ensure_archive};
use crate::stack::StackGuard;

mod downloads;
mod encoding;
mod generation;
mod stack;

pub const AARCHMRS_2025_03_URL: &str = "https://developer.arm.com/-/cdn-downloads/permalink/Exploration-Tools-OS-Machine-Readable-Data/AARCHMRS_BSD/AARCHMRS_OPENSOURCE_A_profile-2025-03.tar.gz";
pub const AARCHMRS_2025_03_FILE: &str = "AARCHMRS_OPENSOURCE_A_profile-2025-03.tar.gz";
pub const AARCHMRS_2025_03_MD5: [u8; 16] = hex_literal::hex!("dcc4850852a18ae0e786ccbe52fafbb0");
pub const AARCHMRS_2025_03_SIZE: u64 = 3517054;
pub const AARCHMRS_INSTRUCTIONS_FILE: &str = "Instructions.json";

pub fn gen_instructions(dest_dir: &Path, cache_dir: &Path) -> Result<(), DownloadError> {
    let archive_path = ensure_archive(cache_dir)?;

    let gz_archive_file = std::fs::File::open(archive_path)?;
    let tar_file = flate2::read::GzDecoder::new(gz_archive_file);
    let mut archive = tar::Archive::new(tar_file);

    let mut json_data = vec![];
    for ent in archive.entries()? {
        let mut ent = ent?;
        let path = ent.path()?;
        if path == Path::new(AARCHMRS_INSTRUCTIONS_FILE) {
            std::io::copy(&mut ent, &mut json_data)?;
            break;
        }
    }
    assert!(!json_data.is_empty());

    let data: aarchmrs_parser::instructions::Instructions =
        serde_json::from_slice(&json_data).expect("failed to parse data");

    let mut encoding_stack = vec![];

    let mut lib_mods = vec![];
    for inst_set in &data.instructions {
        let inst_name = &inst_set.name; // single entry: "A64"
        let inst_id = quote::format_ident!("{inst_name}");
        lib_mods.push(quote::quote! { pub mod #inst_id; });

        let nested_dir = dest_dir.join(inst_name);
        std::fs::create_dir_all(&nested_dir)?;

        let child_mods = walk_instructions_children(
            &mut encoding_stack,
            &inst_set.children,
            &nested_dir,
            &data._meta.license,
        )?;

        let mod_path = dest_dir.join(format!("{inst_name}.rs"));
        std::fs::create_dir_all(dest_dir)?;
        write_mod(&mod_path, &child_mods, &<_>::default(), &data._meta.license)?;
    }

    std::fs::create_dir_all(dest_dir)?;

    let lib_path = dest_dir.join("lib.rs");
    let clippy_allow_pragma = quote::quote! { #[allow(
        non_snake_case, non_camel_case_types, clippy::identity_op, clippy::too_many_arguments, clippy::module_inception
    )]};
    write_mod(
        &lib_path,
        &lib_mods,
        &clippy_allow_pragma,
        &data._meta.license,
    )?;

    Ok(())
}

fn walk_instructions_children<'a, 'b: 'a>(
    stack: &'a mut Vec<&'b Encodeset>,
    children: &'b [InstructionGroupOrInstruction],
    dest_dir: &Path,
    license: &License,
) -> io::Result<Vec<TokenStream>> {
    let mut mods = vec![];
    for child in children {
        match child {
            InstructionGroupOrInstruction::InstructionGroup(instruction_group) => {
                let tokens = walk_group(stack, dest_dir, instruction_group, license)?;
                mods.push(tokens);
            }
            InstructionGroupOrInstruction::Instruction(instruction) => {
                let tokens = walk_instruction(stack, instruction)?;
                mods.push(tokens);
            }
            InstructionGroupOrInstruction::InstructionAlias(_instruction_alias) => {
                // ignore so far
            }
        }
    }
    Ok(mods)
}

fn walk_group<'a, 'b: 'a>(
    stack: &'a mut Vec<&'b Encodeset>,
    dest_dir: &Path,
    instruction_group: &'b InstructionGroup,
    license: &License,
) -> io::Result<TokenStream> {
    let group_name = &instruction_group.name;
    let group_file = dest_dir.join(format!("{group_name}.rs"));

    let group_dir = dest_dir.join(group_name);
    let stack2 = StackGuard::push(stack, &instruction_group.encoding);
    let child_mods = walk_instructions_children(
        stack2.data,
        &instruction_group.children,
        &group_dir,
        license,
    )?;

    std::fs::create_dir_all(dest_dir)?;
    write_mod(&group_file, &child_mods, &<_>::default(), license)?;

    let group_id = quote::format_ident!("{group_name}");
    Ok(quote::quote! { pub mod #group_id; })
}

fn walk_instruction<'a, 'b: 'a>(
    stack: &'a mut Vec<&'b Encodeset>,
    instruction: &'b aarchmrs_parser::instructions::Instruction,
) -> io::Result<TokenStream> {
    let inst_name = instruction.name.as_str();

    // TODO is it true that instruction's children are only aliases?
    let stack2 = StackGuard::push(stack, &instruction.encoding);
    let (inst_bits, should_be) = crate::encoding::flatten_encodeset(stack2.data);
    let inst_func = crate::generation::gen_constructor(inst_name, &inst_bits, should_be.mask);

    let inst_id = quote::format_ident!("{inst_name}");
    Ok(quote::quote! {
        pub mod #inst_id { #inst_func }
    })
}

fn write_mod(
    path: &Path,
    mods: &[TokenStream],
    header: &TokenStream,
    license: &License,
) -> io::Result<()> {
    let contents: syn::File = syn::parse_quote! {
        #header
        #(#mods)*
    };

    let contents = prettyplease::unparse(&contents);
    let header = format!("/* {}\n *\n * {}\n */\n", license.copyright, license.info);
    std::fs::write(path, format!("{header}\n{contents}"))?;
    Ok(())
}
