/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod PLI_r_A1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110110010100001111000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLI_r_A1_RRX";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn PLI_r_A1_RRX(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110110u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b111100000110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod PLI_r_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110110010100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLI_r_A1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_stype_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_stype_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm5_OFFSET: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm5_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn PLI_r_A1(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110110u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod PLD_r_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110111010100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLD_r_A1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_stype_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_stype_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm5_OFFSET: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm5_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn PLD_r_A1(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod PLD_r_A1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110111010100001111000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLD_r_A1_RRX";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn PLD_r_A1_RRX(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b111100000110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod PLDW_r_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110111000100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLDW_r_A1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_stype_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_stype_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm5_OFFSET: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm5_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn PLDW_r_A1(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod PLDW_r_A1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110111000100001111000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PLDW_r_A1_RRX";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rm_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn PLDW_r_A1_RRX(
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b111100000110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
