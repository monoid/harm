/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqcvt_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111111111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqcvt_z_mz4_";
    #[inline]
    pub const fn sqcvt_z_mz4_(
        sz: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | sz.into_inner() << 23u32
                | 0b0110011111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqcvtu_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111111111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011100111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqcvtu_z_mz4_";
    #[inline]
    pub const fn sqcvtu_z_mz4_(
        sz: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | sz.into_inner() << 23u32
                | 0b1110011111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqcvtn_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111111111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100111110000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqcvtn_z_mz4_";
    #[inline]
    pub const fn sqcvtn_z_mz4_(
        sz: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | sz.into_inner() << 23u32
                | 0b0110011111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b10u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqcvtun_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111111111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011100111110000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqcvtun_z_mz4_";
    #[inline]
    pub const fn sqcvtun_z_mz4_(
        sz: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | sz.into_inner() << 23u32
                | 0b1110011111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b10u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqcvt_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111111111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100111110000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqcvt_z_mz4_";
    #[inline]
    pub const fn uqcvt_z_mz4_(
        sz: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | sz.into_inner() << 23u32
                | 0b0110011111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b01u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqcvtn_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111111111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100111110000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqcvtn_z_mz4_";
    #[inline]
    pub const fn uqcvtn_z_mz4_(
        sz: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | sz.into_inner() << 23u32
                | 0b0110011111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b11u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
