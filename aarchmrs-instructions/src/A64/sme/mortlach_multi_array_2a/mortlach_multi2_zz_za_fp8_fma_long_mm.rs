/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlal_za_z8z8w_2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011001110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000000000100000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlal_za_z8z8w_2x2";
    #[inline]
    pub const fn fmlal_za_z8z8w_2x2(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | Zm.into_inner() << 17u32
                | 0b00u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b1000u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
