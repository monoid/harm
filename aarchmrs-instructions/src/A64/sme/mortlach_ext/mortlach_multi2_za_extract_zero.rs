/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movaz_mz_za2_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000001100000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "movaz_mz_za2_1";
    #[inline]
    pub const fn movaz_mz_za2_1(
        Rv: ::aarchmrs_types::BitValue<2>,
        off3: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000001100u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b01010u32 << 8u32
                | off3.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
