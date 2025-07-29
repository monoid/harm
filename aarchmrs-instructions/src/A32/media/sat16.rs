/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SSAT16_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110101000000000111100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSAT16_A1";
    #[inline]
    pub const fn SSAT16_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        sat_imm: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b01101010u32 << 20u32
                | sat_imm.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | 0b11110011u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod USAT16_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110111000000000111100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USAT16_A1";
    #[inline]
    pub const fn USAT16_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        sat_imm: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b01101110u32 << 20u32
                | sat_imm.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | 0b11110011u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
