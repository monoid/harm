/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MRS_T1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111011111111000011111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011111011111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010000011011111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MRS_T1_AS";
    #[inline]
    pub const fn MRS_T1_AS(
        R: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110011111u32 << 21u32
                | R.into_inner() << 20u32
                | 0b11111000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b00000000u32 << 0u32,
        )
    }
}
