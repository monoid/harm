/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MRS_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110000110100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MRS_A1_AS";
    #[inline]
    pub const fn MRS_A1_AS(
        cond: ::aarchmrs_types::BitValue<4>,
        R: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010u32 << 23u32
                | R.into_inner() << 22u32
                | 0b001111u32 << 16u32
                | Rd.into_inner() << 12u32
                | 0b000000000000u32 << 0u32,
        )
    }
}
pub mod MRS_br_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111011111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000000000000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000110000001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MRS_br_A1_AS";
    #[inline]
    pub const fn MRS_br_A1_AS(
        cond: ::aarchmrs_types::BitValue<4>,
        R: ::aarchmrs_types::BitValue<1>,
        M1: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010u32 << 23u32
                | R.into_inner() << 22u32
                | 0b00u32 << 20u32
                | M1.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | 0b001u32 << 9u32
                | M.into_inner() << 8u32
                | 0b00000000u32 << 0u32,
        )
    }
}
pub mod MSR_r_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111110100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MSR_r_A1_AS";
    #[inline]
    pub const fn MSR_r_A1_AS(
        cond: ::aarchmrs_types::BitValue<4>,
        R: ::aarchmrs_types::BitValue<1>,
        mask: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010u32 << 23u32
                | R.into_inner() << 22u32
                | 0b10u32 << 20u32
                | mask.into_inner() << 16u32
                | 0b111100000000u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod MSR_br_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100001111111011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MSR_br_A1_AS";
    #[inline]
    pub const fn MSR_br_A1_AS(
        cond: ::aarchmrs_types::BitValue<4>,
        R: ::aarchmrs_types::BitValue<1>,
        M1: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010u32 << 23u32
                | R.into_inner() << 22u32
                | 0b10u32 << 20u32
                | M1.into_inner() << 16u32
                | 0b1111001u32 << 9u32
                | M.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
