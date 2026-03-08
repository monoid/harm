/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */
use aarchmrs_types::InstructionCode;

use super::Rel64Error;
use crate::reloc::get_bytes_mut;

const MOV_OPCODE_OFFSET: u32 = 29;
const MOV_OPCODE_WIDTH: u32 = 2;

const MOV_OPCODE_MOVN: u32 = 0;
const MOV_OPCODE_MOVZ: u32 = 2;

const MOV_IMM16_OFFSET: u32 = 5;
const MOV_IMM16_WIDTH: u32 = 16;

pub fn mov_w_abs_g0_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    const INST_MASK: u32 = ((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET;

    let target: u16 = value.try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    inst_code |= (target as u32) << MOV_IMM16_OFFSET;
    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn mov_w_abs_g0nc_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    const INST_MASK: u32 = ((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET;

    let target = value as u16;
    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    inst_code |= (target as u32) << MOV_IMM16_OFFSET;
    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn mov_w_abs_g0s_reloc(value: i64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    const INST_MASK: u32 = (((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET)
        | (((1 << MOV_OPCODE_WIDTH) - 1) << MOV_OPCODE_OFFSET);

    let target: i16 = value.try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    if target < 0 {
        inst_code |= ((!target as u16) as u32) << MOV_IMM16_OFFSET;
        inst_code |= MOV_OPCODE_MOVN << MOV_OPCODE_OFFSET;
    } else {
        inst_code |= ((target as u16) as u32) << MOV_IMM16_OFFSET;
        inst_code |= MOV_OPCODE_MOVZ << MOV_OPCODE_OFFSET;
    }
    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn mov_w_abs_g1_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    const INST_MASK: u32 = ((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET;

    let target: u16 = (value >> 16).try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    inst_code |= (target as u32) << MOV_IMM16_OFFSET;
    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn mov_w_abs_g1nc_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    const INST_MASK: u32 = ((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET;

    let target = (value >> 16) as u16;
    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    inst_code |= (target as u32) << MOV_IMM16_OFFSET;
    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn mov_w_abs_g1s_reloc(value: i64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    const INST_MASK: u32 = (((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET)
        | (((1 << MOV_OPCODE_WIDTH) - 1) << MOV_OPCODE_OFFSET);

    let target: i16 = (value >> 16).try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    if target < 0 {
        inst_code |= ((!target as u16) as u32) << MOV_IMM16_OFFSET;
        inst_code |= MOV_OPCODE_MOVN << MOV_OPCODE_OFFSET;
    } else {
        inst_code |= ((target as u16) as u32) << MOV_IMM16_OFFSET;
        inst_code |= MOV_OPCODE_MOVZ << MOV_OPCODE_OFFSET;
    }
    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn mov_w_abs_g2_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    const INST_MASK: u32 = ((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET;

    let target: u16 = (value >> 32).try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    inst_code |= (target as u32) << MOV_IMM16_OFFSET;
    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn mov_w_abs_g2nc_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    const INST_MASK: u32 = ((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET;

    let target = (value >> 32) as u16;
    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    inst_code |= (target as u32) << MOV_IMM16_OFFSET;
    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn mov_w_abs_g2s_reloc(value: i64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    const INST_MASK: u32 = (((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET)
        | (((1 << MOV_OPCODE_WIDTH) - 1) << MOV_OPCODE_OFFSET);

    let target: i16 = (value >> 32).try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    if target < 0 {
        inst_code |= ((!target as u16) as u32) << MOV_IMM16_OFFSET;
        inst_code |= MOV_OPCODE_MOVN << MOV_OPCODE_OFFSET;
    } else {
        inst_code |= ((target as u16) as u32) << MOV_IMM16_OFFSET;
        inst_code |= MOV_OPCODE_MOVZ << MOV_OPCODE_OFFSET;
    }
    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}

pub fn mov_w_abs_g3_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    const INST_MASK: u32 = ((1 << MOV_IMM16_WIDTH) - 1) << MOV_IMM16_OFFSET;

    let target = (value >> 48) as u32;
    let bytes = get_bytes_mut(mem, offset)?;

    let mut inst_code = InstructionCode(*bytes).unpack();
    inst_code &= !INST_MASK;
    inst_code |= target << MOV_IMM16_OFFSET;
    *bytes = InstructionCode::from_u32(inst_code).0;

    Ok(())
}
