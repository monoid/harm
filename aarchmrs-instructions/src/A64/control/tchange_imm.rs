/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TCHANGEB_tc_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111011111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101100101000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000011111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TCHANGEB_tc_imm";
    #[inline]
    pub const fn TCHANGEB_tc_imm(
        op1: ::aarchmrs_types::BitValue<1>,
        imm7: ::aarchmrs_types::BitValue<7>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101100101u32 << 18u32
                | op1.into_inner() << 17u32
                | 0b00000u32 << 12u32
                | imm7.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod TCHANGEF_tc_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111011111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101100100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000011111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TCHANGEF_tc_imm";
    #[inline]
    pub const fn TCHANGEF_tc_imm(
        op1: ::aarchmrs_types::BitValue<1>,
        imm7: ::aarchmrs_types::BitValue<7>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101100100u32 << 18u32
                | op1.into_inner() << 17u32
                | 0b00000u32 << 12u32
                | imm7.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
