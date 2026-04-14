/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TCHANGEB_tc_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111011111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101100001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TCHANGEB_tc_reg";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rd_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rd_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_op1_OFFSET: u32 = 17u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_op1_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn TCHANGEB_tc_reg(
        op1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101100001u32 << 18u32
                | op1.into_inner() << 17u32
                | 0b0000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod TCHANGEF_tc_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111011111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TCHANGEF_tc_reg";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rd_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rd_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_op1_OFFSET: u32 = 17u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_op1_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn TCHANGEF_tc_reg(
        op1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101100000u32 << 18u32
                | op1.into_inner() << 17u32
                | 0b0000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
