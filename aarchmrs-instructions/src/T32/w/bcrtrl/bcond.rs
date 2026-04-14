/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod B_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111000000000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "B_T3";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm11_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm11_WIDTH: u32 = 11u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_J2_OFFSET: u32 = 11u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_J2_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_J1_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_J1_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm6_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm6_WIDTH: u32 = 6u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_S_OFFSET: u32 = 26u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_S_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn B_T3(
        S: ::aarchmrs_types::BitValue<1>,
        cond: ::aarchmrs_types::BitValue<4>,
        imm6: ::aarchmrs_types::BitValue<6>,
        J1: ::aarchmrs_types::BitValue<1>,
        J2: ::aarchmrs_types::BitValue<1>,
        imm11: ::aarchmrs_types::BitValue<11>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | S.into_inner() << 26u32
                | cond.into_inner() << 22u32
                | imm6.into_inner() << 16u32
                | 0b10u32 << 14u32
                | J1.into_inner() << 13u32
                | 0b0u32 << 12u32
                | J2.into_inner() << 11u32
                | imm11.into_inner() << 0u32,
        )
    }
}
