/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod B_T4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111000000000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000000000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "B_T4";
    #[inline]
    pub const fn B_T4(
        S: ::aarchmrs_types::BitValue<1>,
        imm10: ::aarchmrs_types::BitValue<10>,
        J1: ::aarchmrs_types::BitValue<1>,
        J2: ::aarchmrs_types::BitValue<1>,
        imm11: ::aarchmrs_types::BitValue<11>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | S.into_inner() << 26u32
                | imm10.into_inner() << 16u32
                | 0b10u32 << 14u32
                | J1.into_inner() << 13u32
                | 0b1u32 << 12u32
                | J2.into_inner() << 11u32
                | imm11.into_inner() << 0u32,
        )
    }
}
