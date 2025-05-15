/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_za2_z_b1 {
    #[inline]
    pub fn mova_za2_z_b1(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000000000100u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b000u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod mova_za2_z_h1 {
    #[inline]
    pub fn mova_za2_z_h1(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        ZAd: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000001000100u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b000u32 << 3u32
                | u32::from(ZAd.into()) << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod mova_za2_z_w1 {
    #[inline]
    pub fn mova_za2_z_w1(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        ZAd: impl Into<::aarchmrs_types::BitValue<2>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010000100u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b000u32 << 3u32
                | u32::from(ZAd.into()) << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
pub mod mova_za2_z_d1 {
    #[inline]
    pub fn mova_za2_z_d1(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        ZAd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000100u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b000u32 << 3u32
                | u32::from(ZAd.into()) << 0u32,
        )
    }
}
