/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRB_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRB_i_T2";
    #[inline]
    pub const fn STRB_i_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000100100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_i_T2";
    #[inline]
    pub const fn LDRB_i_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod PLD_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000100100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLD_i_T1";
    #[inline]
    pub const fn PLD_i_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod PLDW_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000101100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLDW_i_T1";
    #[inline]
    pub const fn PLDW_i_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod STRH_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRH_i_T2";
    #[inline]
    pub const fn STRH_i_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000101100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_i_T2";
    #[inline]
    pub const fn LDRH_i_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod STR_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STR_i_T3";
    #[inline]
    pub const fn STR_i_T3(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDR_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_i_T3";
    #[inline]
    pub const fn LDR_i_T3(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
