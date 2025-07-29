/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRBT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000000000000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRBT_T1";
    #[inline]
    pub const fn STRBT_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod LDRBT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000000100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRBT_T1";
    #[inline]
    pub const fn LDRBT_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod STRHT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000001000000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRHT_T1";
    #[inline]
    pub const fn STRHT_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod LDRHT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000001100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRHT_T1";
    #[inline]
    pub const fn LDRHT_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod STRT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000010000000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRT_T1";
    #[inline]
    pub const fn STRT_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod LDRT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000010100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRT_T1";
    #[inline]
    pub const fn LDRT_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
