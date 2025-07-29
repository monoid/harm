/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlal_za_z8z8i_4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001100100001001000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlal_za_z8z8i_4xi";
    #[inline]
    pub const fn fmlal_za_z8z8i_4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i4h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        i4l: ::aarchmrs_types::BitValue<2>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011001u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i4h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b010u32 << 4u32
                | i4l.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
