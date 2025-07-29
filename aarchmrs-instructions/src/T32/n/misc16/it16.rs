/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod IT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "IT_T1";
    #[inline]
    pub const fn IT_T1(
        firstcond: ::aarchmrs_types::BitValue<4>,
        mask: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111111u32 << 8u32 | firstcond.into_inner() << 4u32 | mask.into_inner() << 0u32,
        )
    }
}
