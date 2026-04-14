/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VSTMDB_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101001000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTMDB_T2";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VSTMDB_T2(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VSTM_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100100000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTM_T2";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_OFFSET: u32 = 21u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VSTM_T2(
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VSTMDB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101001000000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTMDB_T1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VSTMDB_T1(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod VSTM_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100100000000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTM_T1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_OFFSET: u32 = 21u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VSTM_T1(
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod FSTMDBX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101001000000000101100000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FSTMDBX_T1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn FSTMDBX_T1(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod FSTMIAX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100100000000000101100000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FSTMIAX_T1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_OFFSET: u32 = 21u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn FSTMIAX_T1(
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod VLDMDB_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101001100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDMDB_T2";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VLDMDB_T2(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDM_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100100100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDM_T2";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_OFFSET: u32 = 21u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VLDM_T2(
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDMDB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101001100000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDMDB_T1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VLDMDB_T1(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod VLDM_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100100100000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDM_T1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_OFFSET: u32 = 21u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VLDM_T1(
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod FLDMDBX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101001100000000101100000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FLDMDBX_T1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn FLDMDBX_T1(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod FLDMIAX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100100100000000101100000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FLDMIAX_T1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_OFFSET: u32 = 21u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_W_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn FLDMIAX_T1(
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod VSTR_T1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000000000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTR_T1_H";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VSTR_T1_H(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VSTR_T1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTR_T1_S";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VSTR_T1_S(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VSTR_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000000000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTR_T1_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VSTR_T1_D(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_T1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000100000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_T1_H";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VLDR_T1_H(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_T1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_T1_S";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VLDR_T1_S(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000100000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_T1_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VLDR_T1_D(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_l_T1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000111110000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_l_T1_H";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VLDR_l_T1_H(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b011111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_l_T1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000111110000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_l_T1_S";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VLDR_l_T1_S(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b011111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_l_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000111110000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_l_T1_D";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm8_WIDTH: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_OFFSET: u32 = 12u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vd_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_D_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_OFFSET: u32 = 23u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_U_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn VLDR_l_T1_D(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b011111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
