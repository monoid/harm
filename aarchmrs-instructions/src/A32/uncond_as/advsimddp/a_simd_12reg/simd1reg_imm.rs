/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VMOV_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000100111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A1_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMOV_i_A1_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | cmode.into_inner() << 9u32
                | 0b00001u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000100111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A1_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMOV_i_A1_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | cmode.into_inner() << 9u32
                | 0b00101u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMVN_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000100111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMVN_i_A1_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMVN_i_A1_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | cmode.into_inner() << 9u32
                | 0b00011u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMVN_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000100111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMVN_i_A1_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMVN_i_A1_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | cmode.into_inner() << 9u32
                | 0b00111u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VORR_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000100111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000000100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VORR_i_A1_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VORR_i_A1_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | cmode.into_inner() << 9u32
                | 0b10001u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VORR_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000100111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000000101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VORR_i_A1_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VORR_i_A1_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | cmode.into_inner() << 9u32
                | 0b10101u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VBIC_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000100111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000000100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBIC_i_A1_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VBIC_i_A1_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | cmode.into_inner() << 9u32
                | 0b10011u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VBIC_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000100111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000000101110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBIC_i_A1_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VBIC_i_A1_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | cmode.into_inner() << 9u32
                | 0b10111u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_i_A3_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000110111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A3_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMOV_i_A3_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10u32 << 10u32
                | cmode.into_inner() << 9u32
                | 0b00001u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_i_A3_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000110111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000100001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A3_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMOV_i_A3_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10u32 << 10u32
                | cmode.into_inner() << 9u32
                | 0b00101u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMVN_i_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000110111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000100000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMVN_i_A2_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMVN_i_A2_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10u32 << 10u32
                | cmode.into_inner() << 9u32
                | 0b00011u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMVN_i_A2_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000110111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000100001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMVN_i_A2_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMVN_i_A2_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10u32 << 10u32
                | cmode.into_inner() << 9u32
                | 0b00111u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VORR_i_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000110111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000100100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VORR_i_A2_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VORR_i_A2_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10u32 << 10u32
                | cmode.into_inner() << 9u32
                | 0b10001u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VORR_i_A2_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000110111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000100101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VORR_i_A2_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VORR_i_A2_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10u32 << 10u32
                | cmode.into_inner() << 9u32
                | 0b10101u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VBIC_i_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000110111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000100100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBIC_i_A2_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VBIC_i_A2_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10u32 << 10u32
                | cmode.into_inner() << 9u32
                | 0b10011u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VBIC_i_A2_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000110111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000100101110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBIC_i_A2_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 9u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VBIC_i_A2_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10u32 << 10u32
                | cmode.into_inner() << 9u32
                | 0b10111u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_i_A4_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000110011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A4_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMOV_i_A4_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b11u32 << 10u32
                | cmode.into_inner() << 8u32
                | 0b0001u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_i_A4_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000110011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000110001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A4_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMOV_i_A4_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b11u32 << 10u32
                | cmode.into_inner() << 8u32
                | 0b0101u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMVN_i_A3_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000111011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000110000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMVN_i_A3_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMVN_i_A3_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b110u32 << 9u32
                | cmode.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMVN_i_A3_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000111011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000110001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMVN_i_A3_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_OFFSET: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_cmode_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMVN_i_A3_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        cmode: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b110u32 << 9u32
                | cmode.into_inner() << 8u32
                | 0b0111u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_i_A5_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000111000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A5_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMOV_i_A5_D(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b11100011u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_i_A5_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101110000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000111001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A5_Q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_OFFSET: u32 = 24u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VMOV_i_A5_Q(
        i: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | i.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b000u32 << 19u32
                | imm3.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b11100111u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
