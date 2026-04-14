/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldlit_signed_reserved;
pub mod LDRSB_l_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSB_l_T1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm12_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm12_WIDTH: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn LDRSB_l_T1(
        U: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b0011111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod PLI_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLI_i_T3";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm12_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm12_WIDTH: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn PLI_i_T3(
        U: ::aarchmrs_types::BitValue<1>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b00111111111u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRSH_l_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSH_l_T1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm12_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm12_WIDTH: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn LDRSH_l_T1(
        U: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b0111111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
