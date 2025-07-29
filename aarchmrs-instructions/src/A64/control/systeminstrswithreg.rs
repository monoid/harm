/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod WFET_only_systeminstrswithreg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000110001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "WFET_only_systeminstrswithreg";
    #[inline]
    pub const fn WFET_only_systeminstrswithreg(
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010000001100010000000u32 << 5u32 | Rd.into_inner() << 0u32,
        )
    }
}
pub mod WFIT_only_systeminstrswithreg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000110001000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "WFIT_only_systeminstrswithreg";
    #[inline]
    pub const fn WFIT_only_systeminstrswithreg(
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010000001100010000001u32 << 5u32 | Rd.into_inner() << 0u32,
        )
    }
}
