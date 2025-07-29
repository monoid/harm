/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SETEND_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000000010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011101111110100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETEND_A1";
    #[inline]
    pub const fn SETEND_A1(E: ::aarchmrs_types::BitValue<1>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111000100000001000000u32 << 10u32 | E.into_inner() << 9u32 | 0b000000000u32 << 0u32,
        )
    }
}
pub mod CPS_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPS_A1_AS";
    #[inline]
    pub const fn CPS_A1_AS(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110001000000100000000u32 << 9u32
                | A.into_inner() << 8u32
                | I.into_inner() << 7u32
                | F.into_inner() << 6u32
                | 0b0u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod CPSID_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000011000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPSID_A1_AS";
    #[inline]
    pub const fn CPSID_A1_AS(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110001000011000000000u32 << 9u32
                | A.into_inner() << 8u32
                | I.into_inner() << 7u32
                | F.into_inner() << 6u32
                | 0b0u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod CPSID_A1_ASM {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000011100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPSID_A1_ASM";
    #[inline]
    pub const fn CPSID_A1_ASM(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110001000011100000000u32 << 9u32
                | A.into_inner() << 8u32
                | I.into_inner() << 7u32
                | F.into_inner() << 6u32
                | 0b0u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod CPSIE_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000010000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPSIE_A1_AS";
    #[inline]
    pub const fn CPSIE_A1_AS(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110001000010000000000u32 << 9u32
                | A.into_inner() << 8u32
                | I.into_inner() << 7u32
                | F.into_inner() << 6u32
                | 0b0u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod CPSIE_A1_ASM {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000010100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPSIE_A1_ASM";
    #[inline]
    pub const fn CPSIE_A1_ASM(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110001000010100000000u32 << 9u32
                | A.into_inner() << 8u32
                | I.into_inner() << 7u32
                | F.into_inner() << 6u32
                | 0b0u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
