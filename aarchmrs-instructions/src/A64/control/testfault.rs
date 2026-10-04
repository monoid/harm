/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TFLTZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b01111111000001111111001111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110110000000000000001001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TFLTZ";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_b40_OFFSET: u32 = 19u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_b40_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_b5_OFFSET: u32 = 31u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_b5_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn TFLTZ(
        b5: ::aarchmrs_types::BitValue<1>,
        b40: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            b5.into_inner() << 31u32
                | 0b1110110u32 << 24u32
                | b40.into_inner() << 19u32
                | 0b0000000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b10010u32 << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod TFLTNZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b01111111000001111111001111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110110000000000000001001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TFLTNZ";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_b40_OFFSET: u32 = 19u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_b40_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_b5_OFFSET: u32 = 31u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_b5_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn TFLTNZ(
        b5: ::aarchmrs_types::BitValue<1>,
        b40: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            b5.into_inner() << 31u32
                | 0b1110110u32 << 24u32
                | b40.into_inner() << 19u32
                | 0b0000000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b10011u32 << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
