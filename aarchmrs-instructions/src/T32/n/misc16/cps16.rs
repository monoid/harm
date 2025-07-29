/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SETEND_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111110111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011011001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000010111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETEND_T1";
    #[inline]
    pub const fn SETEND_T1(E: ::aarchmrs_types::BitValue<1>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101101100101u32 << 4u32 | E.into_inner() << 3u32 | 0b000u32 << 0u32,
        )
    }
}
pub mod CPSID_T1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011011001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPSID_T1_AS";
    #[inline]
    pub const fn CPSID_T1_AS(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011011001110u32 << 3u32
                | A.into_inner() << 2u32
                | I.into_inner() << 1u32
                | F.into_inner() << 0u32,
        )
    }
}
pub mod CPSIE_T1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011011001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPSIE_T1_AS";
    #[inline]
    pub const fn CPSIE_T1_AS(
        A: ::aarchmrs_types::BitValue<1>,
        I: ::aarchmrs_types::BitValue<1>,
        F: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011011001100u32 << 3u32
                | A.into_inner() << 2u32
                | I.into_inner() << 1u32
                | F.into_inner() << 0u32,
        )
    }
}
