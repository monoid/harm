/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movaz_z_rza_b {
    #[inline]
    pub fn movaz_z_rza_b(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        off4: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000000000010u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b0001u32 << 9u32
                | u32::from(off4.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod movaz_z_rza_h {
    #[inline]
    pub fn movaz_z_rza_h(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        ZAn: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000001000010u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b0001u32 << 9u32
                | u32::from(ZAn.into()) << 8u32
                | u32::from(off3.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod movaz_z_rza_w {
    #[inline]
    pub fn movaz_z_rza_w(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        ZAn: impl Into<::aarchmrs_types::BitValue<2>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010000010u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b0001u32 << 9u32
                | u32::from(ZAn.into()) << 7u32
                | u32::from(off2.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod movaz_z_rza_d {
    #[inline]
    pub fn movaz_z_rza_d(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        ZAn: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000010u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b0001u32 << 9u32
                | u32::from(ZAn.into()) << 6u32
                | u32::from(o1.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod movaz_z_rza_q {
    #[inline]
    pub fn movaz_z_rza_q(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        ZAn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000011u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b0001u32 << 9u32
                | u32::from(ZAn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
