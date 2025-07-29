/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SBFX_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000111101000000000000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBFX_A1";
    #[inline]
    pub const fn SBFX_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        widthm1: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<4>,
        lsb: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0111101u32 << 21u32
                | widthm1.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | lsb.into_inner() << 7u32
                | 0b101u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod UBFX_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000111111000000000000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UBFX_A1";
    #[inline]
    pub const fn UBFX_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        widthm1: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<4>,
        lsb: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0111111u32 << 21u32
                | widthm1.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | lsb.into_inner() << 7u32
                | 0b101u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
