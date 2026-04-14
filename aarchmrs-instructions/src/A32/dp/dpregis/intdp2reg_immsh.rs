/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TST_r_A1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000100000000000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TST_r_A1_RRX";
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
    pub const FIELD_cond_OFFSET: u32 = 28u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn TST_r_A1_RRX(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b000000000110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod TST_r_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TST_r_A1";
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
    pub const FIELD_cond_OFFSET: u32 = 28u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn TST_r_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod TEQ_r_A1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001100000000000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TEQ_r_A1_RRX";
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
    pub const FIELD_cond_OFFSET: u32 = 28u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn TEQ_r_A1_RRX(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b000000000110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod TEQ_r_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TEQ_r_A1";
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
    pub const FIELD_cond_OFFSET: u32 = 28u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn TEQ_r_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CMP_r_A1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010100000000000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMP_r_A1_RRX";
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
    pub const FIELD_cond_OFFSET: u32 = 28u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn CMP_r_A1_RRX(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b000000000110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CMP_r_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMP_r_A1";
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
    pub const FIELD_cond_OFFSET: u32 = 28u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn CMP_r_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CMN_r_A1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011100000000000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMN_r_A1_RRX";
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
    pub const FIELD_cond_OFFSET: u32 = 28u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn CMN_r_A1_RRX(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010111u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b000000000110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CMN_r_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMN_r_A1";
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
    pub const FIELD_cond_OFFSET: u32 = 28u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cond_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn CMN_r_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010111u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
