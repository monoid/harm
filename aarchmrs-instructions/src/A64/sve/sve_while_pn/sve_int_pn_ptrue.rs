/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptrue_pn_i_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111111111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000111100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ptrue_pn_i_";
    #[inline]
    pub const fn ptrue_pn_i_(
        size: ::aarchmrs_types::BitValue<2>,
        PNd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1000000111100000010u32 << 3u32
                | PNd.into_inner() << 0u32,
        )
    }
}
