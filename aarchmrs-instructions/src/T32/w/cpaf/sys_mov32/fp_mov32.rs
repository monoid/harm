/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VMOV_tos_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111101111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101110000000000000101000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001101111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_tos_T1";
    #[inline]
    pub const fn VMOV_tos_T1(
        Vn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011100000u32 << 20u32
                | Vn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0010000u32 << 0u32,
        )
    }
}
pub mod VMOV_s_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111101111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101110000100000000101000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001101111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_s_T1";
    #[inline]
    pub const fn VMOV_s_T1(
        Vn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011100001u32 << 20u32
                | Vn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0010000u32 << 0u32,
        )
    }
}
