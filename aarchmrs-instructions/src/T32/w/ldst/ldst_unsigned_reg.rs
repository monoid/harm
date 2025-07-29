/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRB_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRB_r_T2";
    #[inline]
    pub const fn STRB_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_r_T2";
    #[inline]
    pub const fn LDRB_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod PLD_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000000100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLD_r_T1";
    #[inline]
    pub const fn PLD_r_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod PLDW_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000001100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLDW_r_T1";
    #[inline]
    pub const fn PLDW_r_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRH_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRH_r_T2";
    #[inline]
    pub const fn STRH_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_r_T2";
    #[inline]
    pub const fn LDRH_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STR_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STR_r_T2";
    #[inline]
    pub const fn STR_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDR_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_r_T2";
    #[inline]
    pub const fn LDR_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b000000u32 << 6u32
                | imm2.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
