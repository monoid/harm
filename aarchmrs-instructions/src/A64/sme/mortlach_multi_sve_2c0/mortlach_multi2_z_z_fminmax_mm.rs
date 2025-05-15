/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmax_mz_zzw_2x2 {
    #[inline]
    pub fn fmax_mz_zzw_2x2(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001000u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod bfmax_mz_zzw_2x2 {
    #[inline]
    pub fn bfmax_mz_zzw_2x2(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001001u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001000u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod fmin_mz_zzw_2x2 {
    #[inline]
    pub fn fmin_mz_zzw_2x2(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001000u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod bfmin_mz_zzw_2x2 {
    #[inline]
    pub fn bfmin_mz_zzw_2x2(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001001u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001000u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod fmaxnm_mz_zzw_2x2 {
    #[inline]
    pub fn fmaxnm_mz_zzw_2x2(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001001u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod bfmaxnm_mz_zzw_2x2 {
    #[inline]
    pub fn bfmaxnm_mz_zzw_2x2(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001001u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001001u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod fminnm_mz_zzw_2x2 {
    #[inline]
    pub fn fminnm_mz_zzw_2x2(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001001u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod bfminnm_mz_zzw_2x2 {
    #[inline]
    pub fn bfminnm_mz_zzw_2x2(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001001u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001001u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod famax_mz_zzw_2x2 {
    #[inline]
    pub fn famax_mz_zzw_2x2(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001010u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod famin_mz_zzw_2x2 {
    #[inline]
    pub fn famin_mz_zzw_2x2(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b010110001010u32 << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
