/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SVC_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SVC_A1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm24_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm24_WIDTH: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_OFFSET: u32 = 28u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn SVC_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        imm24: ::aarchmrs_types::BitValue<24>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b1111u32 << 24u32 | imm24.into_inner() << 0u32,
        )
    }
}
