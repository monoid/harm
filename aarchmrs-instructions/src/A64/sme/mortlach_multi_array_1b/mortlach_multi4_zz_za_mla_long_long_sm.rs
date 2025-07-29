/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzv_4x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001001110000000110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlall_za_zzv_4x1";
    #[inline]
    pub const fn smlall_za_zzv_4x1(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | 0b00u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod usmlall_za_zzv_s4x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000001110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100000000000000000100u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmlall_za_zzv_s4x1";
    #[inline]
    pub const fn usmlall_za_zzv_s4x1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010011u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 5u32
                | U.into_inner() << 4u32
                | 0b010u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod smlsll_za_zzv_4x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001001110000000110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlsll_za_zzv_4x1";
    #[inline]
    pub const fn smlsll_za_zzv_4x1(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | 0b00u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod umlall_za_zzv_4x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001001110000000110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlall_za_zzv_4x1";
    #[inline]
    pub const fn umlall_za_zzv_4x1(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | 0b00u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod sumlall_za_zzv_s4x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000001110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100000000000000000100u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumlall_za_zzv_s4x1";
    #[inline]
    pub const fn sumlall_za_zzv_s4x1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010011u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 5u32
                | U.into_inner() << 4u32
                | 0b010u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod umlsll_za_zzv_4x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001001110000000110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlsll_za_zzv_4x1";
    #[inline]
    pub const fn umlsll_za_zzv_4x1(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | 0b00u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
