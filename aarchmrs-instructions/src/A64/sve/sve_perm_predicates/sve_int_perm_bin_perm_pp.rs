/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zip1_p_pp_ {
    #[inline]
    pub fn zip1_p_pp_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01000u32 << 11u32
                | u32::from(H.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod uzp1_p_pp_ {
    #[inline]
    pub fn uzp1_p_pp_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01001u32 << 11u32
                | u32::from(H.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod trn1_p_pp_ {
    #[inline]
    pub fn trn1_p_pp_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01010u32 << 11u32
                | u32::from(H.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod zip2_p_pp_ {
    #[inline]
    pub fn zip2_p_pp_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01000u32 << 11u32
                | u32::from(H.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod uzp2_p_pp_ {
    #[inline]
    pub fn uzp2_p_pp_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01001u32 << 11u32
                | u32::from(H.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod trn2_p_pp_ {
    #[inline]
    pub fn trn2_p_pp_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01010u32 << 11u32
                | u32::from(H.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
