/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_mz4_ztz_4 {
    #[inline]
    pub fn luti2_mz4_ztz_4(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        D: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000100111u32 << 18u32
                | u32::from(i2.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(size.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(D.into()) << 4u32
                | 0b00u32 << 2u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod luti4_mz4_ztz_4 {
    #[inline]
    pub fn luti4_mz4_ztz_4(
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        D: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000001001101u32 << 17u32
                | u32::from(i1.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(size.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(D.into()) << 4u32
                | 0b00u32 << 2u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
