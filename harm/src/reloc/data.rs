/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use super::{Addr, Rel64Error, calc_delta, get_bytes_mut};

#[inline]
pub fn abs64_reloc(value: u64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;
    *bytes = value.to_le_bytes();
    Ok(())
}

#[inline]
pub fn abs32_reloc(value: i64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target: u32 = try_unsigned_and_signed::<u32, i32>(value)?;
    let bytes = get_bytes_mut(mem, offset)?;
    *bytes = target.to_le_bytes();
    Ok(())
}

#[inline]
pub fn abs16_reloc(value: i64, mem: &mut [u8], offset: usize) -> Result<(), Rel64Error> {
    let target: u16 = try_unsigned_and_signed::<u16, i16>(value)?;
    let bytes = get_bytes_mut(mem, offset)?;
    *bytes = target.to_le_bytes();
    Ok(())
}

#[inline]
pub fn prel64_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    *bytes = calc_delta(base, symbol, offset)?.to_le_bytes();
    Ok(())
}

#[inline]
pub fn prel32_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    let delta = calc_delta(base, symbol, offset)?;
    let value = try_unsigned_and_signed::<u32, i32>(delta)?;
    *bytes = value.to_le_bytes();
    Ok(())
}

#[inline]
pub fn prel16_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    let delta = calc_delta(base, symbol, offset)?;
    let value = try_unsigned_and_signed::<u16, i16>(delta)?;
    *bytes = value.to_le_bytes();
    Ok(())
}

#[inline]
pub fn plt32_reloc(
    base: Addr,
    symbol: Addr,
    mem: &mut [u8],
    offset: usize,
) -> Result<(), Rel64Error> {
    let bytes = get_bytes_mut(mem, offset)?;

    let delta = calc_delta(base, symbol, offset)?;
    // Just like PREL32, but the conversion is more restrictive.
    let value = i32::try_from(delta)?;
    *bytes = value.to_le_bytes();
    Ok(())
}

fn try_unsigned_and_signed<T1, T2>(value: i64) -> Result<T1, Rel64Error>
where
    T1: TryFrom<i64>,
    T2: TryFrom<i64>,
    Rel64Error: From<<T2 as TryFrom<i64>>::Error>,
    T2: AsPrimitive<T1>,
{
    Ok(T1::try_from(value).or_else(|_| T2::try_from(value).map(AsPrimitive::as_primitive))?)
}

// It repeats partially the num-traits crate, but I want the crate to be leaner.
#[allow(clippy::wrong_self_convention)]
trait AsPrimitive<Other> {
    fn as_primitive(self) -> Other;
}

impl AsPrimitive<u32> for i32 {
    fn as_primitive(self) -> u32 {
        self as u32
    }
}

impl AsPrimitive<u16> for i16 {
    fn as_primitive(self) -> u16 {
        self as u16
    }
}
