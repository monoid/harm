/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRD_i_T1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRD_i_T1_post";
    #[inline]
    pub const fn STRD_i_T1_post(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | Rt2.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod LDRD_i_T1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRD_i_T1_post";
    #[inline]
    pub const fn LDRD_i_T1_post(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | Rt2.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
