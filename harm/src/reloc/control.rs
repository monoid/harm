/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_types::InstructionCode;

use super::{Addr, Rel64Error, calc_offset};
use crate::instructions::control::{BranchCondOffset, BranchOffset, TestBranchOffset};
use crate::reloc::get_bytes_mut;

const JUMP_IMM26_OFFSET: u32 = 0u32;
const JUMP_IMM26_WIDTH: u32 = 26u32;

const TST_BR_IMM14_OFFSET: u32 = 5u32;
const TST_BR_IMM14_WIDTH: u32 = 14u32;

const COND_BR_IMM19_OFFSET: u32 = 5u32;
const COND_BR_IMM19_WIDTH: u32 = 19u32;

pub fn jump26_reloc(
    base: Addr,
    target: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let jump_offset64 = calc_offset(base, target, offset)?;
    let jump_offset = BranchOffset::new_i64(jump_offset64)?;
    let bytes = get_bytes_mut(mem, offset)?;
    let code = InstructionCode(*bytes).unpack();

    let jump_offset_bits = jump_offset.bits();

    let code_cleaned = code & !(((1u32 << JUMP_IMM26_WIDTH) - 1) << JUMP_IMM26_OFFSET);
    let code_patched = code_cleaned | ((jump_offset_bits as u32) << JUMP_IMM26_OFFSET);

    *bytes = InstructionCode::from_u32(code_patched).0;

    Ok(())
}

#[inline]
pub fn call26_reloc(
    base: Addr,
    target: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    jump26_reloc(base, target, mem, offset)
}

pub fn tst_br14_reloc(
    base: Addr,
    target: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let tst_offset64 = calc_offset(base, target, offset)?;
    let tst_offset = TestBranchOffset::new_i64(tst_offset64)?;
    let bytes = get_bytes_mut(mem, offset)?;
    let code = InstructionCode(*bytes).unpack();

    let jump_offset_bits = tst_offset.bits();

    let code_cleaned = code & !(((1u32 << TST_BR_IMM14_WIDTH) - 1) << TST_BR_IMM14_OFFSET);
    let code_patched = code_cleaned | ((jump_offset_bits as u32) << TST_BR_IMM14_OFFSET);

    *bytes = InstructionCode::from_u32(code_patched).0;

    Ok(())
}

pub fn cond_br19_reloc(
    base: Addr,
    target: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let cond_offset64 = calc_offset(base, target, offset)?;
    let cond_offset = BranchCondOffset::new_i64(cond_offset64)?;
    let bytes = get_bytes_mut(mem, offset)?;
    let code = InstructionCode(*bytes).unpack();

    let jump_offset_bits = cond_offset.bits();

    let code_cleaned = code & !(((1u32 << COND_BR_IMM19_WIDTH) - 1) << COND_BR_IMM19_OFFSET);
    let code_patched = code_cleaned | ((jump_offset_bits as u32) << COND_BR_IMM19_OFFSET);

    *bytes = InstructionCode::from_u32(code_patched).0;

    Ok(())
}
