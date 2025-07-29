/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movt_r_zt_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000010011000000001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "movt_r_zt_";
    #[inline]
    pub const fn movt_r_zt_(
        off3: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000010011000u32 << 15u32
                | off3.into_inner() << 12u32
                | 0b0011111u32 << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
