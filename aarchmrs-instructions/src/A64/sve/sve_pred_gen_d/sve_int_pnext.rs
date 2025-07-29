/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pnext_p_p_p_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000110011100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "pnext_p_p_p_";
    #[inline]
    pub const fn pnext_p_p_p_(
        size: ::aarchmrs_types::BitValue<2>,
        Pv: ::aarchmrs_types::BitValue<4>,
        Pdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0110011100010u32 << 9u32
                | Pv.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pdn.into_inner() << 0u32,
        )
    }
}
