/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smax_mz_zzw_4x4 {
    #[inline]
    pub fn smax_mz_zzw_4x4(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 18u32
                | 0b0010111000000u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b0u32 << 1u32
                | u32::from(U.into()) << 0u32,
        )
    }
}
pub mod smin_mz_zzw_4x4 {
    #[inline]
    pub fn smin_mz_zzw_4x4(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 18u32
                | 0b0010111000001u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b0u32 << 1u32
                | u32::from(U.into()) << 0u32,
        )
    }
}
pub mod umax_mz_zzw_4x4 {
    #[inline]
    pub fn umax_mz_zzw_4x4(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 18u32
                | 0b0010111000000u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b0u32 << 1u32
                | u32::from(U.into()) << 0u32,
        )
    }
}
pub mod umin_mz_zzw_4x4 {
    #[inline]
    pub fn umin_mz_zzw_4x4(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 18u32
                | 0b0010111000001u32 << 5u32
                | u32::from(Zdn.into()) << 2u32
                | 0b0u32 << 1u32
                | u32::from(U.into()) << 0u32,
        )
    }
}
