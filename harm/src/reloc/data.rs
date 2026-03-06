/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use super::{Rel64Error, calc_offset, get_bytes_mut};

pub fn abs64_reloc(target: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;
    *bytes = target.to_le_bytes();
    Ok(())
}

pub fn abs32_reloc(target: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target: u32 = target.try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;
    *bytes = target.to_le_bytes();
    Ok(())
}

pub fn abs16_reloc(target: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target: u16 = target.try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;
    *bytes = target.to_le_bytes();
    Ok(())
}

pub fn prel64_reloc(
    base: u64,
    target: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    *bytes = calc_offset(base, target, offset)?.to_le_bytes();
    Ok(())
}

pub fn prel32_reloc(
    base: u64,
    target: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    let value: i32 = calc_offset(base, target, offset)?.try_into()?;
    *bytes = value.to_le_bytes();
    Ok(())
}

pub fn prel16_reloc(
    base: u64,
    target: u64,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    let value: i16 = calc_offset(base, target, offset)?.try_into()?;
    *bytes = value.to_le_bytes();
    Ok(())
}
