/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzv_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001001110000000100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlall_za_zzv_1";
    #[inline]
    pub const fn smlall_za_zzv_1(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b001u32 << 10u32
                | Zn.into_inner() << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | 0b0u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod usmlall_za_zzv_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000000010000000100u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmlall_za_zzv_s";
    #[inline]
    pub const fn usmlall_za_zzv_s(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b001u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b001u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod smlsll_za_zzv_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001001110000000100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlsll_za_zzv_1";
    #[inline]
    pub const fn smlsll_za_zzv_1(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b001u32 << 10u32
                | Zn.into_inner() << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | 0b0u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlall_za_zzv_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001001110000000100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlall_za_zzv_1";
    #[inline]
    pub const fn umlall_za_zzv_1(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b001u32 << 10u32
                | Zn.into_inner() << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | 0b0u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlsll_za_zzv_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001001110000000100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlsll_za_zzv_1";
    #[inline]
    pub const fn umlsll_za_zzv_1(
        sz: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b001u32 << 10u32
                | Zn.into_inner() << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | 0b0u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
