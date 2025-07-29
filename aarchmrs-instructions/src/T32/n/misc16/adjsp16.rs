/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_SP_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111110000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_SP_i_T2";
    #[inline]
    pub const fn ADD_SP_i_T2(
        imm7: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101100000u32 << 7u32 | imm7.into_inner() << 0u32,
        )
    }
}
pub mod SUB_SP_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111110000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_SP_i_T1";
    #[inline]
    pub const fn SUB_SP_i_T1(
        imm7: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101100001u32 << 7u32 | imm7.into_inner() << 0u32,
        )
    }
}
