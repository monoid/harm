/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod HINTE_extendedhint {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110110001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "HINTE_extendedhint";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm7_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm7_WIDTH: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_L_OFFSET: u32 = 21u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_L_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn HINTE_extendedhint(
        L: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101010100u32 << 22u32
                | L.into_inner() << 21u32
                | 0b00u32 << 19u32
                | imm3.into_inner() << 16u32
                | 0b0010u32 << 12u32
                | imm7.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
