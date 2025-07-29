/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TST_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TST_i_A1";
    #[inline]
    pub const fn TST_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00110001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod TEQ_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TEQ_i_A1";
    #[inline]
    pub const fn TEQ_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00110011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod CMP_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMP_i_A1";
    #[inline]
    pub const fn CMP_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00110101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod CMN_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMN_i_A1";
    #[inline]
    pub const fn CMN_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00110111u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
