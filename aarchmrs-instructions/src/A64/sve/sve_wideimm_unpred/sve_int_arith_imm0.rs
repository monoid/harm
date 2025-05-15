/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod add_z_zi_ {
    #[inline]
    pub fn add_z_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10000011u32 << 14u32
                | u32::from(sh.into()) << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sub_z_zi_ {
    #[inline]
    pub fn sub_z_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10000111u32 << 14u32
                | u32::from(sh.into()) << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod subr_z_zi_ {
    #[inline]
    pub fn subr_z_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10001111u32 << 14u32
                | u32::from(sh.into()) << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sqadd_z_zi_ {
    #[inline]
    pub fn sqadd_z_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10010u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(sh.into()) << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sqsub_z_zi_ {
    #[inline]
    pub fn sqsub_z_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10011u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(sh.into()) << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqadd_z_zi_ {
    #[inline]
    pub fn uqadd_z_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10010u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(sh.into()) << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqsub_z_zi_ {
    #[inline]
    pub fn uqsub_z_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10011u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(sh.into()) << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
