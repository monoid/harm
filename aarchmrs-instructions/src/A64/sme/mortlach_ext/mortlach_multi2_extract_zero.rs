/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movaz_mz2_za_b1 {
    #[inline]
    pub fn movaz_mz2_za_b1(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000000000110u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b00010u32 << 8u32
                | u32::from(off3.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod movaz_mz2_za_h1 {
    #[inline]
    pub fn movaz_mz2_za_h1(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        ZAn: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000001000110u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b00010u32 << 8u32
                | u32::from(ZAn.into()) << 7u32
                | u32::from(off2.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod movaz_mz2_za_w1 {
    #[inline]
    pub fn movaz_mz2_za_w1(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        ZAn: impl Into<::aarchmrs_types::BitValue<2>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010000110u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b00010u32 << 8u32
                | u32::from(ZAn.into()) << 6u32
                | u32::from(o1.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod movaz_mz2_za_d1 {
    #[inline]
    pub fn movaz_mz2_za_d1(
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        ZAn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000110u32 << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | 0b00010u32 << 8u32
                | u32::from(ZAn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
