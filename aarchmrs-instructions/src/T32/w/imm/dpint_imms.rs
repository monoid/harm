/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_i_T4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_i_T4";
    #[inline]
    pub const fn ADD_i_T4(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b100000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ADD_SP_i_T4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000011010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_SP_i_T4";
    #[inline]
    pub const fn ADD_SP_i_T4(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b10000011010u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ADR_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADR_T3";
    #[inline]
    pub const fn ADR_T3(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b10000011110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod SUB_i_T4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_i_T4";
    #[inline]
    pub const fn SUB_i_T4(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b101010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod SUB_SP_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010101011010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_SP_i_T3";
    #[inline]
    pub const fn SUB_SP_i_T3(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b10101011010u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ADR_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010101011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADR_T2";
    #[inline]
    pub const fn ADR_T2(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b10101011110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
