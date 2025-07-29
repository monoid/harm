/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod PLD_l_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000000111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLD_l_T1";
    #[inline]
    pub const fn PLD_l_T1(
        U: ::aarchmrs_types::BitValue<1>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b00111111111u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_l_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_l_T1";
    #[inline]
    pub const fn LDRB_l_T1(
        U: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b0011111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_l_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000001111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_l_T1";
    #[inline]
    pub const fn LDRH_l_T1(
        U: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b0111111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDR_l_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000010111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_l_T2";
    #[inline]
    pub const fn LDR_l_T2(
        U: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b1011111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
