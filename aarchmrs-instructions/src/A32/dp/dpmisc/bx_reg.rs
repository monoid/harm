/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod BX_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001011111111111100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BX_A1";
    #[inline]
    pub const fn BX_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b000100101111111111110001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
