/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlal_za_z8z8i_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000001000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlal_za_z8z8i_1";
    #[inline]
    pub const fn fmlal_za_z8z8i_1(
        Zm: ::aarchmrs_types::BitValue<4>,
        i4A: ::aarchmrs_types::BitValue<1>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i4B: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        i4C: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011100u32 << 20u32
                | Zm.into_inner() << 16u32
                | i4A.into_inner() << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i4B.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | i4C.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
