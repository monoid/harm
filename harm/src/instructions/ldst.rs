/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDR` and related commands.
//!
//! The `ldr` function returns an instance of `Instruction` for loading a 32-bit or 64-bit register from memory. While
//! `LDR` instruction has different variants with various number of arguments, the `ldr` function has two arguments: a
//! destination register and an "address" that encapsulates the rest of arguments: the base, offsets, extensions, etc.
//! Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The funciton is overloaded for various argument types. For some of them, and `Instruction` trait instance is
//! returned, for others, a `Result` if the aguments need validation. Such arugment combinations have `.unwrap()` in
//! examples.
//!
//! # `LDR`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{ldr, extended, shifted, shifted_by, LdrExtendOption32, LdrShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdrExtendOption32::*;
//!
//! ldr(W1, X2);        // LDR W1, [X2]
//! ldr(W1, (X2,));     // LDR W1, [X2]
//! ldr(W1, (X2, X3));  // LDR W1, [X2, X3] ; n.b. a 32-bit register offset requires an extend speicifer (sxtw or uxtw):
//! ldr(W1, (X2, extended(W3, UXTW))); // ldr w1, [x2, w3, uxtw]
//! ldr(W1, (X2, extended(shifted(W3, LdrShift::Shifted), UXTW))); // ldr w1, [x2, w3, uxtw #2]
//! ldr(W1, (X2, extended(W3, UXTW))); // ldr w1, [x2, w3, uxtw]
//! ldr(X1, (X2, extended(W3, UXTW))); // ldr x1, [x2, w3, uxtw]
//! ldr(X1, (X2, extended(shifted(W3, LdrShift::Shifted), UXTW))); // ldr x1, [x2, w3, uxtw #3]
//! # // ldr(X1, (X2, shifted_by(W3, 3).unwrap())); // ldr x1, [x2, w3, uxtw 3]
//! # // TODO ldr(W1, (X2, (W3, UXTW, 3))) // !!!
//! # // TODO ldr(X1, (X2, uxtw(W3, 2))).unwrap();  // LDR X1, [X2, W3 uxtw 3]
//! # // TODO ldr(X1, (X2, sxtw(W3, 2))).unwrap();  // LDR X1, [X2, W3 sxtw 3]
//! # // TODO ldr(X1, (X2, uxtw(W3, 2).unwrap()));  // LDR X1, [X2, W3 uxtw 3]
//! # // TODO ldr(X1, (X2, sxtw(W3, 2).unwrap()));  // LDR X1, [X2, W3 sxtw 3]
//! # // TODO uxtw/sxtw functions
//! ```
//!
//! Please note, that `uxtw` and `sxtw` can be used only with 32-bit register, and shift can be only either 0 or 2. The
//! `lsl` and `sxtx` can be used only with 64-bit registers, and while they produce different bit patterns, they are
//! equivalent; shift can be only 0 or 3.
//!
//! # `LDR`: Register base with immediate offset
//!
//! LDR with register base with immediate offset has an unsigned offset aligned by destination register size.
//! For example, if the desination register is `W1`, the offset has be aligned by 4 bytes (two lower bits are clear),
//! and if it is `X1`, the offset has to be aligned by 8 bytes (three lower bits are clear).  The offset has 12
//! significan bytes available.
//!
//! You may also a `u32` offset value, and a error is returned if the value doesn't fit the offset pattern.
//!
//! Examples:
//! ```ignore
//! let word_aligned_offset: UBitValue<12, 2> = ...;
//! let dword_aligned_offset: UBitValue<12, 3> = ...;
//!
//! ldr(W1, (X2, offset as u32)).unwrap(),
//! ldr(X1, (X2, offset as u32)).unwrap(),
//! ldr(W1, (X2, word_aligned_offset)),
//! ldr(X1, (X2, dword_aligned_offset)),
//! ```
//!
//! # `LDR`: PC base with immediate offset
//!
//! An immediate signed offset of `SBitValue<12, 2>` is added to `PC`, i.e. the offset is relative to the instruction's
//! address.  The 2-bit alignment is the same for both 32-bit and 64-bit variants.
//!
//! ```ignore
//! let bit_offset: PcOffset = ...;
//! let raw_offset: i32 = ...;
//!
//! ldr(W1, pc(bit_offset)),
//! ldr(W1, (Pc, bit_offset)),
//! ldr(W1, (Pc, raw_offset)).unwrap(),
//! ```
//!
//! # `LDUR`: Register base with unaligned immediate offset
//!
//! `LDUR` is similar to `LDR` with register base with immediate offset, but its offset is **signed** 9 bit wide. For
//! convenience, a `SBitValue<9>` value can be used with `ldr` as well, and both signed and unsigned raw values can be
//! encoded as LDUR if they fit into the range and cannot be encoded with `LDR` with immediate offset (TODO it makes the
//! code little bit more complex, unless we use an enum).
//!
//! ```ignore
//! let bit_offset: SBitOffset<9> = ...;
//! let raw_offset: i32 = ...;
//!
//! ldr(W1, (X2, bit_offset)),
//! ldur(W1, (X2, bit_offset)),
//! ldr(X1, (X2, bit_offset)),
//! ldur(X1, (X2, bit_offset)),
//! ldr(W1, (X2, raw_offset)).unwap(),
//! ldur(W1, (X2, raw_offset)).unwap(),
//! ldr(X1, (X2, raw_offset)).unwap(),
//! ldur(X1, (X2, raw_offset)).unwap(),
//! ```

mod ldr;
mod shift_extend;

pub use self::shift_extend::*;
pub use self::ldr::*;
use crate::{
    bits::{SBitValue, UBitValue},
    register::RegOrSp64,
};

pub type PcOffset = SBitValue<19>;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pc(PcOffset);

pub type ScaledOffset32 = UBitValue<12, 2>;
pub type ScaledOffset64 = UBitValue<12, 3>;

pub type UnscaledOffset = SBitValue<9>;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Unscaled(pub RegOrSp64, pub UnscaledOffset);

pub const fn unscaled(base: RegOrSp64, offset: UnscaledOffset) -> Unscaled {
    Unscaled(base, offset)
}

pub struct Load<Dst, Addr> {
    pub dst: Dst,
    pub addr: Addr,
}

pub trait MakeLoad<Dst, Addr> {
    type Output;
    fn new(dst: Dst, addr: Addr) -> Self::Output;
}

