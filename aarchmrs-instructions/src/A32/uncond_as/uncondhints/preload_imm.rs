/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod PLI_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100010100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLI_i_A1";
    #[inline]
    pub const fn PLI_i_A1(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod PLD_l_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101010111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000010000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLD_l_A1";
    #[inline]
    pub const fn PLD_l_A1(
        U: ::aarchmrs_types::BitValue<1>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b10111111111u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod PLD_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101010100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLD_i_A1";
    #[inline]
    pub const fn PLD_i_A1(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod PLDW_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101000100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLDW_i_A1";
    #[inline]
    pub const fn PLDW_i_A1(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
