/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlall_za32_z8z8i_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlall_za32_z8z8i_1";
    #[inline]
    pub const fn fmlall_za32_z8z8i_1(
        Zm: ::aarchmrs_types::BitValue<4>,
        i4h: ::aarchmrs_types::BitValue<1>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i4l: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010100u32 << 20u32
                | Zm.into_inner() << 16u32
                | i4h.into_inner() << 15u32
                | Rv.into_inner() << 13u32
                | i4l.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b000u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
