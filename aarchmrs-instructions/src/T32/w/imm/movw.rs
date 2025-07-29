/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOV_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_i_T3";
    #[inline]
    pub const fn MOV_i_T3(
        i: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b100100u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod MOVT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVT_T1";
    #[inline]
    pub const fn MOVT_T1(
        i: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b101100u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
