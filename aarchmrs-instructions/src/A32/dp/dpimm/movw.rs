/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOV_i_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_i_A2";
    #[inline]
    pub const fn MOV_i_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00110000u32 << 20u32
                | imm4.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod MOVT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVT_A1";
    #[inline]
    pub const fn MOVT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00110100u32 << 20u32
                | imm4.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
