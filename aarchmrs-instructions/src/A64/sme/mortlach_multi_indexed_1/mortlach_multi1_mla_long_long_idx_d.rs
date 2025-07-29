/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000001000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlall_za_zzi_d";
    #[inline]
    pub const fn smlall_za_zzi_d(
        Zm: ::aarchmrs_types::BitValue<4>,
        i3h: ::aarchmrs_types::BitValue<1>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3l: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011000u32 << 20u32
                | Zm.into_inner() << 16u32
                | i3h.into_inner() << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b000u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod smlsll_za_zzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000001000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001100000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlsll_za_zzi_d";
    #[inline]
    pub const fn smlsll_za_zzi_d(
        Zm: ::aarchmrs_types::BitValue<4>,
        i3h: ::aarchmrs_types::BitValue<1>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3l: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011000u32 << 20u32
                | Zm.into_inner() << 16u32
                | i3h.into_inner() << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b010u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlall_za_zzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000001000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001100000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlall_za_zzi_d";
    #[inline]
    pub const fn umlall_za_zzi_d(
        Zm: ::aarchmrs_types::BitValue<4>,
        i3h: ::aarchmrs_types::BitValue<1>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3l: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011000u32 << 20u32
                | Zm.into_inner() << 16u32
                | i3h.into_inner() << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b100u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlsll_za_zzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000001000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001100000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlsll_za_zzi_d";
    #[inline]
    pub const fn umlsll_za_zzi_d(
        Zm: ::aarchmrs_types::BitValue<4>,
        i3h: ::aarchmrs_types::BitValue<1>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3l: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011000u32 << 20u32
                | Zm.into_inner() << 16u32
                | i3h.into_inner() << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b110u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
