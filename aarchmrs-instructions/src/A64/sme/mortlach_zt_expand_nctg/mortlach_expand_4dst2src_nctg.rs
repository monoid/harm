/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti4_mz4_ztmz2_4 {
    #[inline]
    pub fn luti4_mz4_ztmz2_4(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        D: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000001001101100u32 << 14u32
                | u32::from(size.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(D.into()) << 4u32
                | 0b00u32 << 2u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
