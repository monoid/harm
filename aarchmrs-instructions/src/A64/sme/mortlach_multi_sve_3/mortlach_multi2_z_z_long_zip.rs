/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zip_mz_zz_2q {
    #[inline]
    pub fn zip_mz_zz_2q(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001001u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b110101u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod uzp_mz_zz_2q {
    #[inline]
    pub fn uzp_mz_zz_2q(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001001u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b110101u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
