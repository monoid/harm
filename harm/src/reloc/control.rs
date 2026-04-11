/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use super::{Addr, Rel64Error, calc_delta, patch_instruction_bits};
use crate::instructions::control::{BranchCondOffset, BranchOffset, TestBranchOffset};
use crate::reloc::get_bytes_mut;

const JUMP_IMM26_OFFSET: u32 = 0u32;
const JUMP_IMM26_WIDTH: u32 = 26u32;

const TST_BR_IMM14_OFFSET: u32 = 5u32;
const TST_BR_IMM14_WIDTH: u32 = 14u32;

const COND_BR_IMM19_OFFSET: u32 = 5u32;
const COND_BR_IMM19_WIDTH: u32 = 19u32;

#[inline]
pub fn jump26_reloc(
    base: Addr,
    target: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let jump_offset64 = calc_delta(base, target, offset)?;
    let jump_offset = BranchOffset::new_i64(jump_offset64)?;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_instruction_bits::<JUMP_IMM26_OFFSET, JUMP_IMM26_WIDTH>(bytes, jump_offset.bits());
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

#[inline]
pub fn tst_br14_reloc(
    base: Addr,
    target: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let tst_offset64 = calc_delta(base, target, offset)?;
    let tst_offset = TestBranchOffset::new_i64(tst_offset64)?;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_instruction_bits::<TST_BR_IMM14_OFFSET, TST_BR_IMM14_WIDTH>(bytes, tst_offset.bits());
    Ok(())
}

#[inline]
pub fn cond_br19_reloc(
    base: Addr,
    target: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let cond_offset64 = calc_delta(base, target, offset)?;
    let cond_offset = BranchCondOffset::new_i64(cond_offset64)?;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_instruction_bits::<COND_BR_IMM19_OFFSET, COND_BR_IMM19_WIDTH>(bytes, cond_offset.bits());
    Ok(())
}
