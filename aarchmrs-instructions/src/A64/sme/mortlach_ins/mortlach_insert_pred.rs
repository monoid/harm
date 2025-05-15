/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_za_p_rz_b {
    #[inline]
    pub fn mova_za_p_rz_b(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        off4: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000000000000u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(off4.into()) << 0u32,
        )
    }
}
pub mod mova_za_p_rz_h {
    #[inline]
    pub fn mova_za_p_rz_h(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAd: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000001000000u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(ZAd.into()) << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod mova_za_p_rz_w {
    #[inline]
    pub fn mova_za_p_rz_w(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAd: impl Into<::aarchmrs_types::BitValue<2>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010000000u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(ZAd.into()) << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod mova_za_p_rz_d {
    #[inline]
    pub fn mova_za_p_rz_d(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAd: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000000u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(ZAd.into()) << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
pub mod mova_za_p_rz_q {
    #[inline]
    pub fn mova_za_p_rz_q(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000001u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(ZAd.into()) << 0u32,
        )
    }
}
