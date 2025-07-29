/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldst_signed_reg_reserved;
pub mod LDRSB_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSB_r_T2";
    #[inline]
    pub const fn LDRSB_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod PLI_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLI_r_T1";
    #[inline]
    pub const fn PLI_r_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRSH_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSH_r_T2";
    #[inline]
    pub const fn LDRSH_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
