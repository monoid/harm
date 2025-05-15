/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fscale_mz_zzw_2x2 {
    #[inline]
    pub fn fscale_mz_zzw_2x2(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001100u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod bfscale_mz_zzw_2x2 {
    #[inline]
    pub fn bfscale_mz_zzw_2x2(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001001u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001100u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
