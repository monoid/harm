/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod B_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "B_A1";
    #[inline]
    pub const fn B_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        imm24: ::aarchmrs_types::BitValue<24>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b1010u32 << 24u32 | imm24.into_inner() << 0u32,
        )
    }
}
pub mod BL_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001011000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BL_i_A1";
    #[inline]
    pub const fn BL_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        imm24: ::aarchmrs_types::BitValue<24>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b1011u32 << 24u32 | imm24.into_inner() << 0u32,
        )
    }
}
pub mod BL_i_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BL_i_A2";
    #[inline]
    pub const fn BL_i_A2(
        H: ::aarchmrs_types::BitValue<1>,
        imm24: ::aarchmrs_types::BitValue<24>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111101u32 << 25u32 | H.into_inner() << 24u32 | imm24.into_inner() << 0u32,
        )
    }
}
