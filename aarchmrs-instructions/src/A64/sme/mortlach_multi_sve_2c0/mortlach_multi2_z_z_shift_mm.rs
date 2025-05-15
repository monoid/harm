/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod srshl_mz_zzw_2x2 {
    #[inline]
    pub fn srshl_mz_zzw_2x2(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110010001u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | u32::from(U.into()) << 0u32,
        )
    }
}
pub mod urshl_mz_zzw_2x2 {
    #[inline]
    pub fn urshl_mz_zzw_2x2(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110010001u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | u32::from(U.into()) << 0u32,
        )
    }
}
