/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod wrffr_f_p_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "wrffr_f_p_";
    #[inline]
    pub const fn wrffr_f_p_(
        Pn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101001010001001000u32 << 9u32 | Pn.into_inner() << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
