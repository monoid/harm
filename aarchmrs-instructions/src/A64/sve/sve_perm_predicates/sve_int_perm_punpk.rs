/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod punpklo_p_p_ {
    #[inline]
    pub fn punpklo_p_p_(
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001010011000u32 << 17u32
                | u32::from(H.into()) << 16u32
                | 0b0100000u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod punpkhi_p_p_ {
    #[inline]
    pub fn punpkhi_p_p_(
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001010011000u32 << 17u32
                | u32::from(H.into()) << 16u32
                | 0b0100000u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
