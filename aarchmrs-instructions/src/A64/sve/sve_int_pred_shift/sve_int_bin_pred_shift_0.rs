/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod asr_z_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "asr_z_p_zi_";
    #[inline]
    pub const fn asr_z_p_zi_(
        tszh: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | tszh.into_inner() << 22u32
                | 0b00000u32 << 17u32
                | U.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | tszl.into_inner() << 8u32
                | imm3.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod lsl_z_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000000111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "lsl_z_p_zi_";
    #[inline]
    pub const fn lsl_z_p_zi_(
        tszh: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | tszh.into_inner() << 22u32
                | 0b000011100u32 << 13u32
                | Pg.into_inner() << 10u32
                | tszl.into_inner() << 8u32
                | imm3.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod asrd_z_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000001001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "asrd_z_p_zi_";
    #[inline]
    pub const fn asrd_z_p_zi_(
        tszh: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | tszh.into_inner() << 22u32
                | 0b000100100u32 << 13u32
                | Pg.into_inner() << 10u32
                | tszl.into_inner() << 8u32
                | imm3.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sqshl_z_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000001101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqshl_z_p_zi_";
    #[inline]
    pub const fn sqshl_z_p_zi_(
        tszh: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | tszh.into_inner() << 22u32
                | 0b00011u32 << 17u32
                | U.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | tszl.into_inner() << 8u32
                | imm3.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod srshr_z_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000011001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "srshr_z_p_zi_";
    #[inline]
    pub const fn srshr_z_p_zi_(
        tszh: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | tszh.into_inner() << 22u32
                | 0b00110u32 << 17u32
                | U.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | tszl.into_inner() << 8u32
                | imm3.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sqshlu_z_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000011111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqshlu_z_p_zi_";
    #[inline]
    pub const fn sqshlu_z_p_zi_(
        tszh: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | tszh.into_inner() << 22u32
                | 0b001111100u32 << 13u32
                | Pg.into_inner() << 10u32
                | tszl.into_inner() << 8u32
                | imm3.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod lsr_z_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "lsr_z_p_zi_";
    #[inline]
    pub const fn lsr_z_p_zi_(
        tszh: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | tszh.into_inner() << 22u32
                | 0b00000u32 << 17u32
                | U.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | tszl.into_inner() << 8u32
                | imm3.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqshl_z_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000001101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqshl_z_p_zi_";
    #[inline]
    pub const fn uqshl_z_p_zi_(
        tszh: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | tszh.into_inner() << 22u32
                | 0b00011u32 << 17u32
                | U.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | tszl.into_inner() << 8u32
                | imm3.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod urshr_z_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000011001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "urshr_z_p_zi_";
    #[inline]
    pub const fn urshr_z_p_zi_(
        tszh: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | tszh.into_inner() << 22u32
                | 0b00110u32 << 17u32
                | U.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | tszl.into_inner() << 8u32
                | imm3.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
