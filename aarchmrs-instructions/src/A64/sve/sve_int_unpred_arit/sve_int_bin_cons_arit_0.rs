/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod add_z_zz_ {
    #[inline]
    pub fn add_z_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sub_z_zz_ {
    #[inline]
    pub fn sub_z_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod addpt_z_zz_ {
    #[inline]
    pub fn addpt_z_zz_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100111u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b000010u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod subpt_z_zz_ {
    #[inline]
    pub fn subpt_z_zz_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100111u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b000011u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqadd_z_zz_ {
    #[inline]
    pub fn sqadd_z_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b00010u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqsub_z_zz_ {
    #[inline]
    pub fn sqsub_z_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b00011u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uqadd_z_zz_ {
    #[inline]
    pub fn uqadd_z_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b00010u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uqsub_z_zz_ {
    #[inline]
    pub fn uqsub_z_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b00011u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
