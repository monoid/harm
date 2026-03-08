/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use super::{Addr, Rel64Error, calc_offset, get_bytes_mut};

pub fn abs64_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;
    *bytes = value.to_le_bytes();
    Ok(())
}

pub fn abs32_reloc(value: i64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target: i32 = value.try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;
    *bytes = target.to_le_bytes();
    Ok(())
}

pub fn abs16_reloc(value: i64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target: i16 = value.try_into()?;
    let bytes = get_bytes_mut(mem, offset)?;
    *bytes = target.to_le_bytes();
    Ok(())
}

pub fn prel64_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    *bytes = calc_offset(base, symbol, offset)?.to_le_bytes();
    Ok(())
}

pub fn prel32_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    let value: i32 = calc_offset(base, symbol, offset)?.try_into()?;
    *bytes = value.to_le_bytes();
    Ok(())
}

pub fn prel16_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    let value: i16 = calc_offset(base, symbol, offset)?.try_into()?;
    *bytes = value.to_le_bytes();
    Ok(())
}
