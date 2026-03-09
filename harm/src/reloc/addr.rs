/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_types::InstructionCode;

use super::{Addr, Rel64Error, cond_br19_reloc};
use crate::instructions::dpimm::{AdrOffset, AdrpOffset};
use crate::instructions::ldst::{ScaledOffset16, ScaledOffset32, ScaledOffset64, ScaledOffset128};
use crate::reloc::get_bytes_mut;
use crate::reloc::{calc_offset, calc_page_offset};

const ADR_ADRP_IMMHI_OFFSET: u32 = 5u32;
const ADR_ADRP_IMMHI_WIDTH: u32 = 19u32;
const ADR_ADRP_IMMLO_OFFSET: u32 = 29u32;
const ADR_ADRP_IMMLO_WIDTH: u32 = 2u32;

const ADD_IMM12_OFFSET: u32 = 10u32;
const ADD_IMM12_WIDTH: u32 = 12u32;

const LDST16_IMM12_OFFSET: u32 = 10u32;
// The value width is 12 bits to be compatible with ADRP, even though 13-bit values (12 bit value + 1 bit shift) can be
// encoded in the instruction.
const LDST16_IMM12_VALUE_WIDTH: u32 = 12u32;
const LDST16_IMM12_CLEAR_WIDTH: u32 = 12u32;

const LDST32_IMM12_OFFSET: u32 = 10u32;
// The value width is 12 bits to be compatible with ADRP, even though 14-bit values (12 bit value + 2 bit shift) can be
// encoded in the instruction.
const LDST32_IMM12_VALUE_WIDTH: u32 = 12u32;
const LDST32_IMM12_CLEAR_WIDTH: u32 = 12u32;

const LDST64_IMM12_OFFSET: u32 = 10u32;
// The value width is 12 bits to be compatible with ADRP, even though 15-bit values (12 bit value + 3 bit shift) can be
// encoded in the instruction.
const LDST64_IMM12_VALUE_WIDTH: u32 = 12u32;
const LDST64_IMM12_CLEAR_WIDTH: u32 = 12u32;

const LDST128_IMM12_OFFSET: u32 = 10u32;
// The value width is 12 bits to be compatible with ADRP, even though 16-bit values (12 bit value + 4 bit shift) can be
// encoded in the instruction.
const LDST128_IMM12_VALUE_WIDTH: u32 = 12u32;
const LDST128_IMM12_CLEAR_WIDTH: u32 = 12u32;

#[inline]
pub fn ld_prel_lo19_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    cond_br19_reloc(base, symbol, mem, offset)
}

pub fn adr_prel_lo21_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    let delta = calc_offset(base, symbol, offset)?;
    let delta = AdrOffset::new_i64(delta)?;
    patch_adr_adrp(bytes, delta.bits());

    Ok(())
}

#[inline]
pub fn adrp_prel_pg_hi21_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const VALUE_MASK: u32 = (1 << (ADR_ADRP_IMMHI_WIDTH + ADR_ADRP_IMMLO_WIDTH)) - 1;

    let bytes = get_bytes_mut(mem, offset)?;

    let delta = calc_page_offset(base, symbol, offset)?;
    let delta = AdrpOffset::new_i64(delta).map_err(Rel64Error::InvalidBits)?;
    patch_adr_adrp(bytes, delta.bits() & VALUE_MASK);

    Ok(())
}

#[inline]
pub fn adrp_prel_pg_hi21_nc_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const VALUE_MASK: u32 = (1 << (ADR_ADRP_IMMHI_WIDTH + ADR_ADRP_IMMLO_WIDTH)) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // It is not checked.
    let delta = calc_page_offset(base, symbol, offset)? >> 12;

    patch_adr_adrp(bytes, (delta as u32) & VALUE_MASK);
    Ok(())
}

#[inline]
pub fn add_abs_lo_12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const MASK: u32 = (1 << ADD_IMM12_WIDTH) - 1;

    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !(MASK << ADD_IMM12_OFFSET);
    inst_code |= (symbol as u32 & MASK) << ADD_IMM12_OFFSET;

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

#[inline]
pub fn ldst8_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    add_abs_lo_12_nc_reloc(symbol, mem, offset)
}

pub fn ldst16_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const CLEAR_MASK: u32 = (1 << LDST16_IMM12_CLEAR_WIDTH) - 1;
    const VALUE_MASK: u32 = (1 << LDST16_IMM12_VALUE_WIDTH) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // ... a linker should check that the value of X is aligned to a multiple of the datum size.
    let symbol_bits = ScaledOffset16::new(symbol as u32 & VALUE_MASK)?;
    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !(CLEAR_MASK << LDST16_IMM12_OFFSET);
    inst_code |= symbol_bits.bits() << LDST16_IMM12_OFFSET;

    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn ldst32_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const CLEAR_MASK: u32 = (1 << LDST32_IMM12_CLEAR_WIDTH) - 1;
    const VALUE_MASK: u32 = (1 << LDST32_IMM12_VALUE_WIDTH) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // ... a linker should check that the value of X is aligned to a multiple of the datum size.
    let symbol_bits = ScaledOffset32::new(symbol as u32 & VALUE_MASK)?;
    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !(CLEAR_MASK << LDST32_IMM12_OFFSET);
    inst_code |= symbol_bits.bits() << LDST32_IMM12_OFFSET;

    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn ldst64_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const CLEAR_MASK: u32 = (1 << LDST64_IMM12_CLEAR_WIDTH) - 1;
    const VALUE_MASK: u32 = (1 << LDST64_IMM12_VALUE_WIDTH) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // ... a linker should check that the value of X is aligned to a multiple of the datum size.
    let symbol_bits = ScaledOffset64::new(symbol as u32 & VALUE_MASK)?;
    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !(CLEAR_MASK << LDST64_IMM12_OFFSET);
    inst_code |= symbol_bits.bits() << LDST64_IMM12_OFFSET;

    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn ldst128_abs_lo12_nc_reloc(
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    const CLEAR_MASK: u32 = (1 << LDST128_IMM12_CLEAR_WIDTH) - 1;
    const VALUE_MASK: u32 = (1 << LDST128_IMM12_VALUE_WIDTH) - 1;

    let bytes = get_bytes_mut(mem, offset)?;
    // ... a linker should check that the value of X is aligned to a multiple of the datum size.
    let symbol_bits = ScaledOffset128::new(symbol as u32 & VALUE_MASK)?;
    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !(CLEAR_MASK << LDST128_IMM12_OFFSET);
    inst_code |= symbol_bits.bits() << LDST128_IMM12_OFFSET;

    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}
