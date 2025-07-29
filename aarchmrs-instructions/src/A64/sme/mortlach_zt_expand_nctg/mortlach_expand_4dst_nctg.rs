/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_mz4_ztz_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111001100110000001100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000100111001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "luti2_mz4_ztz_4";
    #[inline]
    pub const fn luti2_mz4_ztz_4(
        i2: ::aarchmrs_types::BitValue<2>,
        size: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        D: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000100111u32 << 18u32
                | i2.into_inner() << 16u32
                | 0b10u32 << 14u32
                | size.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Zn.into_inner() << 5u32
                | D.into_inner() << 4u32
                | 0b00u32 << 2u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod luti4_mz4_ztz_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111101100110000001100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000100110101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "luti4_mz4_ztz_4";
    #[inline]
    pub const fn luti4_mz4_ztz_4(
        i1: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        D: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000001001101u32 << 17u32
                | i1.into_inner() << 16u32
                | 0b10u32 << 14u32
                | size.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Zn.into_inner() << 5u32
                | D.into_inner() << 4u32
                | 0b00u32 << 2u32
                | Zd.into_inner() << 0u32,
        )
    }
}
