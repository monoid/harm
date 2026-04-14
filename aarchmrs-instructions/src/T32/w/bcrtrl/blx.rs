/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod BL_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111000000000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BL_i_T2";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_H_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_H_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm10L_OFFSET: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm10L_WIDTH: u32 = 10u32;
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
    pub const FIELD_imm10H_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm10H_WIDTH: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_S_OFFSET: u32 = 26u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_S_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn BL_i_T2(
        S: ::aarchmrs_types::BitValue<1>,
        imm10H: ::aarchmrs_types::BitValue<10>,
        J1: ::aarchmrs_types::BitValue<1>,
        J2: ::aarchmrs_types::BitValue<1>,
        imm10L: ::aarchmrs_types::BitValue<10>,
        H: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | S.into_inner() << 26u32
                | imm10H.into_inner() << 16u32
                | 0b11u32 << 14u32
                | J1.into_inner() << 13u32
                | 0b0u32 << 12u32
                | J2.into_inner() << 11u32
                | imm10L.into_inner() << 1u32
                | H.into_inner() << 0u32,
        )
    }
}
