/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smopa_za_pp_zz_32 {
    #[inline]
    pub fn smopa_za_pp_zz_32(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000100u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b000u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod sumopa_za_pp_zz_32 {
    #[inline]
    pub fn sumopa_za_pp_zz_32(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000101u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b000u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod usmopa_za_pp_zz_32 {
    #[inline]
    pub fn usmopa_za_pp_zz_32(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001100u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b000u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod umopa_za_pp_zz_32 {
    #[inline]
    pub fn umopa_za_pp_zz_32(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001101u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b000u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod smops_za_pp_zz_32 {
    #[inline]
    pub fn smops_za_pp_zz_32(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000100u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b100u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod sumops_za_pp_zz_32 {
    #[inline]
    pub fn sumops_za_pp_zz_32(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000101u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b100u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod usmops_za_pp_zz_32 {
    #[inline]
    pub fn usmops_za_pp_zz_32(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001100u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b100u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod umops_za_pp_zz_32 {
    #[inline]
    pub fn umops_za_pp_zz_32(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001101u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b100u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
