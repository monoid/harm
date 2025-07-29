/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfcvt_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfcvt_z_mz2_";
    #[inline]
    pub const fn bfcvt_z_mz2_(
        Zn: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000101100000111000u32 << 10u32
                | Zn.into_inner() << 6u32
                | N.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvt_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvt_z_mz2_";
    #[inline]
    pub const fn fcvt_z_mz2_(
        Zn: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100000111000u32 << 10u32
                | Zn.into_inner() << 6u32
                | N.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod bfcvtn_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfcvtn_z_mz2_";
    #[inline]
    pub const fn bfcvtn_z_mz2_(
        Zn: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000101100000111000u32 << 10u32
                | Zn.into_inner() << 6u32
                | N.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtn_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtn_z_mz2_";
    #[inline]
    pub const fn fcvtn_z_mz2_(
        Zn: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100000111000u32 << 10u32
                | Zn.into_inner() << 6u32
                | N.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
