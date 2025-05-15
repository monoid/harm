/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqxtnb_z_zz_ {
    #[inline]
    pub fn sqxtnb_z_zz_(
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | 0b0000100u32 << 12u32
                | u32::from(U.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqxtunb_z_zz_ {
    #[inline]
    pub fn sqxtunb_z_zz_(
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | 0b00001010u32 << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqxtnt_z_zz_ {
    #[inline]
    pub fn sqxtnt_z_zz_(
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | 0b0000100u32 << 12u32
                | u32::from(U.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqxtunt_z_zz_ {
    #[inline]
    pub fn sqxtunt_z_zz_(
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | 0b00001010u32 << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uqxtnb_z_zz_ {
    #[inline]
    pub fn uqxtnb_z_zz_(
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | 0b0000100u32 << 12u32
                | u32::from(U.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uqxtnt_z_zz_ {
    #[inline]
    pub fn uqxtnt_z_zz_(
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | 0b0000100u32 << 12u32
                | u32::from(U.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
