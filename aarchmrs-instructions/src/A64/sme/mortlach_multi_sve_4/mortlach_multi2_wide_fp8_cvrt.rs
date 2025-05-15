/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod f1cvt_mz2_z8_ {
    #[inline]
    pub fn f1cvt_mz2_z8_(
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100110111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | u32::from(L.into()) << 0u32,
        )
    }
}
pub mod bf1cvt_mz2_z8_ {
    #[inline]
    pub fn bf1cvt_mz2_z8_(
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000101100110111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | u32::from(L.into()) << 0u32,
        )
    }
}
pub mod f2cvt_mz2_z8_ {
    #[inline]
    pub fn f2cvt_mz2_z8_(
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110100110111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | u32::from(L.into()) << 0u32,
        )
    }
}
pub mod bf2cvt_mz2_z8_ {
    #[inline]
    pub fn bf2cvt_mz2_z8_(
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000111100110111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | u32::from(L.into()) << 0u32,
        )
    }
}
pub mod f1cvtl_mz2_z8_ {
    #[inline]
    pub fn f1cvtl_mz2_z8_(
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100110111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | u32::from(L.into()) << 0u32,
        )
    }
}
pub mod bf1cvtl_mz2_z8_ {
    #[inline]
    pub fn bf1cvtl_mz2_z8_(
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000101100110111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | u32::from(L.into()) << 0u32,
        )
    }
}
pub mod f2cvtl_mz2_z8_ {
    #[inline]
    pub fn f2cvtl_mz2_z8_(
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110100110111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | u32::from(L.into()) << 0u32,
        )
    }
}
pub mod bf2cvtl_mz2_z8_ {
    #[inline]
    pub fn bf2cvtl_mz2_z8_(
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000111100110111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | u32::from(L.into()) << 0u32,
        )
    }
}
