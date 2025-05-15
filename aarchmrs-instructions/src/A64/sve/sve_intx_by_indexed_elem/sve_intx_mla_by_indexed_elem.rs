/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mla_z_zzzi_h {
    #[inline]
    pub fn mla_z_zzzi_h(
        i3h: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001000u32 << 23u32
                | u32::from(i3h.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(i3l.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b00001u32 << 11u32
                | u32::from(S.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod mla_z_zzzi_s {
    #[inline]
    pub fn mla_z_zzzi_s(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i2.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b00001u32 << 11u32
                | u32::from(S.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod mla_z_zzzi_d {
    #[inline]
    pub fn mla_z_zzzi_d(
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i1.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b00001u32 << 11u32
                | u32::from(S.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod mls_z_zzzi_h {
    #[inline]
    pub fn mls_z_zzzi_h(
        i3h: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001000u32 << 23u32
                | u32::from(i3h.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(i3l.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b00001u32 << 11u32
                | u32::from(S.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod mls_z_zzzi_s {
    #[inline]
    pub fn mls_z_zzzi_s(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i2.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b00001u32 << 11u32
                | u32::from(S.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod mls_z_zzzi_d {
    #[inline]
    pub fn mls_z_zzzi_d(
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i1.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b00001u32 << 11u32
                | u32::from(S.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
