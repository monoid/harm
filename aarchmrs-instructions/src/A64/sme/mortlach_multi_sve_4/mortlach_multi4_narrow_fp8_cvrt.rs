/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcvt_z8_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001101001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvt_z8_mz4_";
    #[inline]
    pub const fn fcvt_z8_mz4_(
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100110100111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtn_z8_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001101001110000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtn_z8_mz4_";
    #[inline]
    pub const fn fcvtn_z8_mz4_(
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100110100111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b01u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
