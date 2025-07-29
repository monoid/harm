/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptest__p_p_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100001000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101010100001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ptest__p_p_";
    #[inline]
    pub const fn ptest__p_p_(
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010101000011u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b00000u32 << 0u32,
        )
    }
}
