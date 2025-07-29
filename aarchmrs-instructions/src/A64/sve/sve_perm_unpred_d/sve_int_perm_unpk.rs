/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sunpklo_z_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001100000011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sunpklo_z_z_";
    #[inline]
    pub const fn sunpklo_z_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b110000001110u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sunpkhi_z_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001100010011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sunpkhi_z_z_";
    #[inline]
    pub const fn sunpkhi_z_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b110001001110u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uunpklo_z_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001100100011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uunpklo_z_z_";
    #[inline]
    pub const fn uunpklo_z_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b110010001110u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uunpkhi_z_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001100110011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uunpkhi_z_z_";
    #[inline]
    pub const fn uunpkhi_z_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b110011001110u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
