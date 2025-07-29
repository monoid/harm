/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqcvtn_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111010000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001100010100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqcvtn_z_mz2_";
    #[inline]
    pub const fn sqcvtn_z_mz2_(
        U: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001100010100u32 << 12u32
                | U.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqcvtun_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001100010101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqcvtun_z_mz2_";
    #[inline]
    pub const fn sqcvtun_z_mz2_(
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100010100110001010100u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqcvtn_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111010000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001100010100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqcvtn_z_mz2_";
    #[inline]
    pub const fn uqcvtn_z_mz2_(
        U: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001100010100u32 << 12u32
                | U.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
