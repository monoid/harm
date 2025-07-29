/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FMOV_S_floatimm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000001111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_S_floatimm";
    #[inline]
    pub const fn FMOV_S_floatimm(
        imm8: ::aarchmrs_types::BitValue<8>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | imm8.into_inner() << 13u32
                | 0b10000000u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_D_floatimm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000001111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_D_floatimm";
    #[inline]
    pub const fn FMOV_D_floatimm(
        imm8: ::aarchmrs_types::BitValue<8>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | imm8.into_inner() << 13u32
                | 0b10000000u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_H_floatimm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000001111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_H_floatimm";
    #[inline]
    pub const fn FMOV_H_floatimm(
        imm8: ::aarchmrs_types::BitValue<8>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | imm8.into_inner() << 13u32
                | 0b10000000u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
