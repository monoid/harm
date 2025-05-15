/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod addha_za_pp_z_32 {
    #[inline]
    pub fn addha_za_pp_z_32(
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010010000u32 << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b000u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod addha_za_pp_z_64 {
    #[inline]
    pub fn addha_za_pp_z_64(
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011010000u32 << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b00u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod addva_za_pp_z_32 {
    #[inline]
    pub fn addva_za_pp_z_32(
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010010001u32 << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b000u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod addva_za_pp_z_64 {
    #[inline]
    pub fn addva_za_pp_z_64(
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011010001u32 << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b00u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
