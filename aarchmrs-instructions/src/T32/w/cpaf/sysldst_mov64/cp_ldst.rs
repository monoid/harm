/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDC_l_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010111111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100000111110101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDC_l_T1";
    #[inline]
    pub const fn LDC_l_T1(
        P: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110110u32 << 25u32
                | P.into_inner() << 24u32
                | U.into_inner() << 23u32
                | 0b0u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1111101011110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod STC_T1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000000000101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STC_T1_off";
    #[inline]
    pub const fn STC_T1_off(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b01011110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod STC_T1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100001000000101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STC_T1_post";
    #[inline]
    pub const fn STC_T1_post(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b01011110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod STC_T1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101001000000101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STC_T1_pre";
    #[inline]
    pub const fn STC_T1_pre(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b01011110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod STC_T1_unind {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100100000000101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STC_T1_unind";
    #[inline]
    pub const fn STC_T1_unind(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011001000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b01011110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod LDC_i_T1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000100000101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDC_i_T1_off";
    #[inline]
    pub const fn LDC_i_T1_off(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b01011110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod LDC_i_T1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100001100000101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDC_i_T1_post";
    #[inline]
    pub const fn LDC_i_T1_post(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b01011110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod LDC_i_T1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101001100000101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDC_i_T1_pre";
    #[inline]
    pub const fn LDC_i_T1_pre(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b01011110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod LDC_i_T1_unind {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100100100000101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDC_i_T1_unind";
    #[inline]
    pub const fn LDC_i_T1_unind(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011001001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b01011110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
