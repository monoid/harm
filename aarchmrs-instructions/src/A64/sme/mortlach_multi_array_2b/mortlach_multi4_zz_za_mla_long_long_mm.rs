/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000111001110001111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlall_za_zzw_4x4";
    #[inline]
    pub const fn smlall_za_zzw_4x4(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b000000u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod usmlall_za_zzw_s4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000111001110001111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000010000000000000100u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmlall_za_zzw_s4x4";
    #[inline]
    pub const fn usmlall_za_zzw_s4x4(
        Zm: ::aarchmrs_types::BitValue<3>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b000010u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod smlsll_za_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000111001110001111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000010000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlsll_za_zzw_4x4";
    #[inline]
    pub const fn smlsll_za_zzw_4x4(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b000100u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod umlall_za_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000111001110001111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000010000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlall_za_zzw_4x4";
    #[inline]
    pub const fn umlall_za_zzw_4x4(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b001000u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod umlsll_za_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000111001110001111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000010000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlsll_za_zzw_4x4";
    #[inline]
    pub const fn umlsll_za_zzw_4x4(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b001100u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
