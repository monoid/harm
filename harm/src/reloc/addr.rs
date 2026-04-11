/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_types::InstructionCode;

use super::{Addr, Rel64Error, cond_br19_reloc};
use crate::instructions::dpimm::{AdrOffset, AdrpOffset};
use crate::instructions::ldst::{ScaledOffset16, ScaledOffset32, ScaledOffset64, ScaledOffset128};
use crate::reloc::{calc_delta, calc_page_offset};
use crate::reloc::{get_bytes_mut, patch_instruction_bits};

const ADR_ADRP_IMMHI_OFFSET: u32 = 5u32;
const ADR_ADRP_IMMHI_WIDTH: u32 = 19u32;
const ADR_ADRP_IMMLO_OFFSET: u32 = 29u32;
const ADR_ADRP_IMMLO_WIDTH: u32 = 2u32;

const ADD_IMM12_OFFSET: u32 = 10u32;
const ADD_IMM12_WIDTH: u32 = 12u32;

const LDST16_IMM12_OFFSET: u32 = 10u32;
// The value width is 12 bits to be compatible with ADRP, even though 13-bit values (12 bit value + 1 bit shift) can be
// encoded in the instruction.
const LDST16_IMM12_WIDTH: u32 = 12u32;

const LDST32_IMM12_OFFSET: u32 = 10u32;
// The value width is 12 bits to be compatible with ADRP, even though 14-bit values (12 bit value + 2 bit shift) can be
// encoded in the instruction.
const LDST32_IMM12_WIDTH: u32 = 12u32;

const LDST64_IMM12_OFFSET: u32 = 10u32;
// The value width is 12 bits to be compatible with ADRP, even though 15-bit values (12 bit value + 3 bit shift) can be
// encoded in the instruction.
const LDST64_IMM12_WIDTH: u32 = 12u32;

const LDST128_IMM12_OFFSET: u32 = 10u32;
// The value width is 12 bits to be compatible with ADRP, even though 16-bit values (12 bit value + 4 bit shift) can be
// encoded in the instruction.
const LDST128_IMM12_WIDTH: u32 = 12u32;

#[inline]
pub fn ld_prel_lo19_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    cond_br19_reloc(base, symbol, mem, offset)
}

#[inline]
pub fn adr_prel_lo21_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    let delta = calc_delta(base, symbol, offset)?;
    let delta = AdrOffset::new_i64(delta)?;
    patch_adr_adrp(bytes, delta.bits());

    Ok(())
}

#[inline]
pub fn adr_prel_pg_hi21_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const MASK: u32 = (1 << (ADR_ADRP_IMMHI_WIDTH + ADR_ADRP_IMMLO_WIDTH)) - 1;

    let bytes = get_bytes_mut(mem, offset)?;

    let delta = calc_page_offset(base, symbol, offset)?;
    let delta = AdrpOffset::new_i64(delta).map_err(Rel64Error::InvalidBits)?;
    patch_adr_adrp(bytes, delta.bits() & MASK);

    Ok(())
}

#[inline]
pub fn adr_prel_pg_hi21_nc_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const MASK: u32 = (1 << (ADR_ADRP_IMMHI_WIDTH + ADR_ADRP_IMMLO_WIDTH)) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // It is not checked.
    let delta = calc_page_offset(base, symbol, offset)? >> 12;

    patch_adr_adrp(bytes, (delta as u32) & MASK);
    Ok(())
}

#[inline]
pub fn add_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;
    patch_instruction_bits::<ADD_IMM12_OFFSET, ADD_IMM12_WIDTH>(bytes, symbol as u32);

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

#[inline]
pub fn ldst8_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    add_abs_lo12_nc_reloc(symbol, mem, offset)
}

#[inline]
pub fn ldst16_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const MASK: u32 = (1 << LDST16_IMM12_WIDTH) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // ... a linker should check that the value of X is aligned to a multiple of the datum size.
    let symbol_bits = ScaledOffset16::new(symbol as u32 & MASK)?;
    patch_instruction_bits::<LDST16_IMM12_OFFSET, LDST16_IMM12_WIDTH>(bytes, symbol_bits.bits());
    Ok(())
}

#[inline]
pub fn ldst32_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const MASK: u32 = (1 << LDST32_IMM12_WIDTH) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // ... a linker should check that the value of X is aligned to a multiple of the datum size.
    let symbol_bits = ScaledOffset32::new(symbol as u32 & MASK)?;
    patch_instruction_bits::<LDST32_IMM12_OFFSET, LDST32_IMM12_WIDTH>(bytes, symbol_bits.bits());
    Ok(())
}

#[inline]
pub fn ldst64_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const MASK: u32 = (1 << LDST64_IMM12_WIDTH) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // ... a linker should check that the value of X is aligned to a multiple of the datum size.
    let symbol_bits = ScaledOffset64::new(symbol as u32 & MASK)?;
    patch_instruction_bits::<LDST64_IMM12_OFFSET, LDST64_IMM12_WIDTH>(bytes, symbol_bits.bits());
    Ok(())
}

#[inline]
pub fn ldst128_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const MASK: u32 = (1 << LDST128_IMM12_WIDTH) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // ... a linker should check that the value of X is aligned to a multiple of the datum size.
    let symbol_bits = ScaledOffset128::new(symbol as u32 & MASK)?;
    patch_instruction_bits::<LDST128_IMM12_OFFSET, LDST128_IMM12_WIDTH>(bytes, symbol_bits.bits());
    Ok(())
}
