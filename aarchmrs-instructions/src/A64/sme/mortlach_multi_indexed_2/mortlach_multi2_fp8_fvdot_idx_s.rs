/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fvdotb_za32_z8z8i_2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100000110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fvdotb_za32_z8z8i_2xi";
    #[inline]
    pub const fn fvdotb_za32_z8z8i_2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2h: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b01u32 << 11u32
                | i2h.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b00u32 << 4u32
                | i2l.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fvdott_za32_z8z8i_2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100000110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fvdott_za32_z8z8i_2xi";
    #[inline]
    pub const fn fvdott_za32_z8z8i_2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2h: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b01u32 << 11u32
                | i2h.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b01u32 << 4u32
                | i2l.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
