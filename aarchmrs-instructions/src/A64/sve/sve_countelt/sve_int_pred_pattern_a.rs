/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incb_r_rs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "incb_r_rs_";
    #[inline]
    pub const fn incb_r_rs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001000011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod decb_r_rs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "decb_r_rs_";
    #[inline]
    pub const fn decb_r_rs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001000011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod inch_r_rs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "inch_r_rs_";
    #[inline]
    pub const fn inch_r_rs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001000111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod dech_r_rs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011100001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "dech_r_rs_";
    #[inline]
    pub const fn dech_r_rs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001000111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod incw_r_rs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100101100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "incw_r_rs_";
    #[inline]
    pub const fn incw_r_rs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod decw_r_rs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100101100001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "decw_r_rs_";
    #[inline]
    pub const fn decw_r_rs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod incd_r_rs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "incd_r_rs_";
    #[inline]
    pub const fn incd_r_rs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod decd_r_rs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100111100001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "decd_r_rs_";
    #[inline]
    pub const fn decd_r_rs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
