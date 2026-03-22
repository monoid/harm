/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */
use aarchmrs_types::InstructionCode;

use super::{Rel64Error, calc_offset};
use crate::{bits::SBitValue, reloc::get_bytes_mut};

const MOV_OPCODE_OFFSET: u32 = 29;
const MOV_OPCODE_WIDTH: u32 = 2;

const MOV_OPCODE_MOVN: u32 = 0;
const MOV_OPCODE_MOVZ: u32 = 2;

const MOV_IMM16_OFFSET: u32 = 5;
const MOV_IMM16_WIDTH: u32 = 16;

#[inline]
pub fn movw_uabs_g0_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target: u16 = value.try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_mov(target, bytes);
    Ok(())
}

#[inline]
pub fn movw_uabs_g0_nc_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target = value as u16;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_mov(target, bytes);
    Ok(())
}

#[inline]
pub fn movw_uabs_g1_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target: u16 = (value >> 16).try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_mov(target, bytes);
    Ok(())
}

#[inline]
pub fn movw_uabs_g1_nc_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target = (value >> 16) as u16;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_mov(target, bytes);
    Ok(())
}

#[inline]
pub fn movw_uabs_g2_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target: u16 = (value >> 32).try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_mov(target, bytes);
    Ok(())
}

#[inline]
pub fn movw_uabs_g2_nc_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target = (value >> 32) as u16;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_mov(target, bytes);
    Ok(())
}

#[inline]
pub fn movw_sabs_g0_reloc(value: i64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target = SBitValue::<17>::new_i64(value)?;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_mov_signed(target, bytes);
    Ok(())
}

#[inline]
pub fn movw_sabs_g1_reloc(value: i64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    movw_sabs_g0_reloc(value >> 16, mem, offset)
}

#[inline]
pub fn movw_sabs_g2_reloc(value: i64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    movw_sabs_g0_reloc(value >> 32, mem, offset)
}

#[inline]
pub fn movw_uabs_g3_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target = (value >> 48) as u16;
    let bytes = get_bytes_mut(mem, offset)?;
    patch_mov(target, bytes);
    Ok(())
}

#[inline]
pub fn movw_prel_g0_reloc(
    base: u64,
    value: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let delta = calc_offset(base, value, offset)?;
    // TODO it seems the range check is different.
    movw_sabs_g0_reloc(delta, mem, offset)
}

#[inline]
pub fn movw_prel_g0_nc_reloc(
    base: u64,
    value: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let delta = calc_offset(base, value, offset)?;
    movw_uabs_g0_nc_reloc(delta as _, mem, offset)
}

#[inline]
pub fn movw_prel_g1_reloc(
    base: u64,
    value: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let delta = calc_offset(base, value, offset)?;
    movw_sabs_g1_reloc(delta, mem, offset)
}

#[inline]
pub fn movw_prel_g1_nc_reloc(
    base: u64,
    value: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let delta = calc_offset(base, value, offset)?;
    movw_uabs_g1_nc_reloc(delta as _, mem, offset)
}

#[inline]
pub fn movw_prel_g2_reloc(
    base: u64,
    value: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let delta = calc_offset(base, value, offset)?;
    movw_sabs_g2_reloc(delta, mem, offset)
}

#[inline]
pub fn movw_prel_g2_nc_reloc(
    base: u64,
    value: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let delta = calc_offset(base, value, offset)?;
    movw_uabs_g2_nc_reloc(delta as _, mem, offset)
}

#[inline]
pub fn movw_prel_g3_reloc(
    base: u64,
    value: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let delta = calc_offset(base, value, offset)?;
    movw_uabs_g3_reloc(delta as _, mem, offset)
}

fn patch_mov(target: u16, bytes: &mut [u8; 4]) {
    const INST_MASK: u32 = ((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    inst_code |= (target as u32) << MOV_IMM16_OFFSET;
    *bytes = InstructionCode::from_u32(inst_code).0;
}

// 16 bit of value field and 1 bit for sign encoded as MOVZ/MOVN instruction.
fn patch_mov_signed(target: SBitValue<17>, bytes: &mut [u8; 4]) {
    let target = target.bits();
    let sign = target >> 16;

    const INST_MASK: u32 = (((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET)
        | (((1 << MOV_OPCODE_WIDTH) - 1) << MOV_OPCODE_OFFSET);

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    if sign != 0 {
        inst_code |= ((!target as u16) as u32) << MOV_IMM16_OFFSET;
        inst_code |= MOV_OPCODE_MOVN << MOV_OPCODE_OFFSET;
    } else {
        inst_code |= ((target as u16) as u32) << MOV_IMM16_OFFSET;
        inst_code |= MOV_OPCODE_MOVZ << MOV_OPCODE_OFFSET;
    }
    *bytes = InstructionCode::from_u32(inst_code).0;
}
