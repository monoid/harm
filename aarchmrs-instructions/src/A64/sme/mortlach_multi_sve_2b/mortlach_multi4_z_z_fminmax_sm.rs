/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmax_mz_zzv_4x1 {
    #[inline]
    pub fn fmax_mz_zzv_4x1(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10101001000u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod bfmax_mz_zzv_4x1 {
    #[inline]
    pub fn bfmax_mz_zzv_4x1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10101001000u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod fmin_mz_zzv_4x1 {
    #[inline]
    pub fn fmin_mz_zzv_4x1(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10101001000u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
pub mod bfmin_mz_zzv_4x1 {
    #[inline]
    pub fn bfmin_mz_zzv_4x1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10101001000u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
pub mod fmaxnm_mz_zzv_4x1 {
    #[inline]
    pub fn fmaxnm_mz_zzv_4x1(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10101001001u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod bfmaxnm_mz_zzv_4x1 {
    #[inline]
    pub fn bfmaxnm_mz_zzv_4x1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10101001001u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod fminnm_mz_zzv_4x1 {
    #[inline]
    pub fn fminnm_mz_zzv_4x1(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10101001001u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
pub mod bfminnm_mz_zzv_4x1 {
    #[inline]
    pub fn bfminnm_mz_zzv_4x1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10101001001u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
