/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CPS_T2_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPS_T2_AS";
    #[inline]
    pub const fn CPS_T2_AS(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111010111110000001u32 << 8u32
                | A.into_inner() << 7u32
                | I.into_inner() << 6u32
                | F.into_inner() << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod CPSID_T2_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000011000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPSID_T2_AS";
    #[inline]
    pub const fn CPSID_T2_AS(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111010111110000110u32 << 8u32
                | A.into_inner() << 7u32
                | I.into_inner() << 6u32
                | F.into_inner() << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod CPSID_T2_ASM {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000011100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPSID_T2_ASM";
    #[inline]
    pub const fn CPSID_T2_ASM(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111010111110000111u32 << 8u32
                | A.into_inner() << 7u32
                | I.into_inner() << 6u32
                | F.into_inner() << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod CPSIE_T2_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPSIE_T2_AS";
    #[inline]
    pub const fn CPSIE_T2_AS(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111010111110000100u32 << 8u32
                | A.into_inner() << 7u32
                | I.into_inner() << 6u32
                | F.into_inner() << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod CPSIE_T2_ASM {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000010100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPSIE_T2_ASM";
    #[inline]
    pub const fn CPSIE_T2_ASM(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111010111110000101u32 << 8u32
                | A.into_inner() << 7u32
                | I.into_inner() << 6u32
                | F.into_inner() << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
