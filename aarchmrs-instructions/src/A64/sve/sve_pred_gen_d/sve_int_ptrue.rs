/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptrue_p_s_ {
    #[inline]
    pub fn ptrue_p_s_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b01100u32 << 17u32
                | u32::from(S.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod ptrues_p_s_ {
    #[inline]
    pub fn ptrues_p_s_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b01100u32 << 17u32
                | u32::from(S.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
