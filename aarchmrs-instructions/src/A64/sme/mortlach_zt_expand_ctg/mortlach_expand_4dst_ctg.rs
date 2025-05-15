/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_mz4_ztz_1 {
    #[inline]
    pub fn luti2_mz4_ztz_1(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000100011u32 << 18u32
                | u32::from(i2.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(size.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod luti4_mz4_ztz_1 {
    #[inline]
    pub fn luti4_mz4_ztz_1(
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000001000101u32 << 17u32
                | u32::from(i1.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(size.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
