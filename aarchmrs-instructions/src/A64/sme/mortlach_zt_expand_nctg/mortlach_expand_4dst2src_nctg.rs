/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti4_mz4_ztmz2_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100110000101100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000100110110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "luti4_mz4_ztmz2_4";
    #[inline]
    pub const fn luti4_mz4_ztmz2_4(
        size: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000001001101100u32 << 14u32
                | size.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | D.into_inner() << 4u32
                | 0b00u32 << 2u32
                | Zd.into_inner() << 0u32,
        )
    }
}
