/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod REV_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110101111110000111100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV_A1";
    #[inline]
    pub const fn REV_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b011010111111u32 << 16u32
                | Rd.into_inner() << 12u32
                | 0b11110011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod REV16_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110101111110000111110110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV16_A1";
    #[inline]
    pub const fn REV16_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b011010111111u32 << 16u32
                | Rd.into_inner() << 12u32
                | 0b11111011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RBIT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110111111110000111100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RBIT_A1";
    #[inline]
    pub const fn RBIT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b011011111111u32 << 16u32
                | Rd.into_inner() << 12u32
                | 0b11110011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod REVSH_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110111111110000111110110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REVSH_A1";
    #[inline]
    pub const fn REVSH_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b011011111111u32 << 16u32
                | Rd.into_inner() << 12u32
                | 0b11111011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
