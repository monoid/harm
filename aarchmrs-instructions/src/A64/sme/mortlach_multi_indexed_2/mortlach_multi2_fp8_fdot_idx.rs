/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fdot_za_z8z8i_2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000000110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000000000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fdot_za_z8z8i_2xi";
    #[inline]
    pub const fn fdot_za_z8z8i_2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        i3l: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3h.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b10u32 << 4u32
                | i3l.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fvdot_za_z8z8i_2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000000110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000001000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fvdot_za_z8z8i_2xi";
    #[inline]
    pub const fn fvdot_za_z8z8i_2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        i3l: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i3h.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b10u32 << 4u32
                | i3l.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
