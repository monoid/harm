/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_types::InstructionCode;

use super::{Rel64Error, cond_br19_reloc};
use crate::instructions::dpimm::{AdrOffset, AdrpOffset};
use crate::reloc::calc_offset;
use crate::reloc::get_bytes_mut;

const ADR_ADRP_IMMHI_OFFSET: u32 = 5u32;
const ADR_ADRP_IMMHI_WIDTH: u32 = 19u32;
const ADR_ADRP_IMMLO_OFFSET: u32 = 29u32;
const ADR_ADRP_IMMLO_WIDTH: u32 = 2u32;

const ADD_IMM12_OFFSET: u32 = 10u32;
const ADD_IMM12_WIDTH: u32 = 12u32;

#[inline]
pub fn ld_prel_lo19_reloc(
    base: u64,
    target: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    cond_br19_reloc(base, target, mem, offset)
}

pub fn adr_prel_lo21_reloc(
    base: u64,
    target: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    let delta = calc_offset(base, target, offset)?;
    let delta = AdrOffset::new_i64(delta)?;
    patch_adr_adrp(bytes, delta.bits());

    Ok(())
}

#[inline]
pub fn adrp_prel_pg_hi21_reloc(
    base: u64,
    target: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const VALUE_MASK: u32 = (1 << (ADR_ADRP_IMMHI_WIDTH + ADR_ADRP_IMMLO_WIDTH)) - 1;

    let bytes = get_bytes_mut(mem, offset)?;

    let delta = calc_offset(base, target, offset)? & !0xFFF;
    let delta = AdrpOffset::new_i64(delta).map_err(Rel64Error::InvalidBits)?;
    patch_adr_adrp(bytes, delta.bits() & VALUE_MASK);

    Ok(())
}

#[inline]
pub fn adrp_prel_pg_hi21_nc_reloc(
    base: u64,
    target: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const VALUE_MASK: u32 = (1 << (ADR_ADRP_IMMHI_WIDTH + ADR_ADRP_IMMLO_WIDTH)) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // It is not checked.
    let delta = calc_offset(base, target, offset)? >> 12;

    patch_adr_adrp(bytes, (delta as u32) & VALUE_MASK);
    Ok(())
}

#[inline]
pub fn add_abs_lo_12_nc_reloc(
    base: u64,
    target: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const MASK: u32 = (1 << ADD_IMM12_WIDTH) - 1;

    let bytes = get_bytes_mut(mem, offset)?;

    // delta is semantically a signed value, but we are using here lower bits only,
    // so we use an unsigned value.
    let delta = calc_offset(base, target, offset)? as u32;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !(MASK << ADD_IMM12_OFFSET);
    inst_code |= (delta & MASK) << ADD_IMM12_OFFSET;

    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

fn patch_adr_adrp(mem: &mut [u8; 4], checked_value: u32) {
    const INST_MASK: u32 = ((1 << ADR_ADRP_IMMHI_WIDTH) - 1) << ADR_ADRP_IMMHI_OFFSET
        | ((1 << ADR_ADRP_IMMLO_WIDTH) - 1) << ADR_ADRP_IMMLO_OFFSET;

    let lo = checked_value & ((1 << ADR_ADRP_IMMLO_WIDTH) - 1);
    let hi = checked_value >> ADR_ADRP_IMMLO_WIDTH;

    let mut inst_code = InstructionCode(*mem).unpack();
    inst_code &= !INST_MASK;
    inst_code |= (hi << ADR_ADRP_IMMHI_OFFSET) | (lo << ADR_ADRP_IMMLO_OFFSET);

    *mem = InstructionCode::from_u32(inst_code).0;
}
