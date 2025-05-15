/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmmla_z32_zz8z8_ {
    #[inline]
    pub fn fmmla_z32_zz8z8_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmmla_z16_zz8z8_ {
    #[inline]
    pub fn fmmla_z16_zz8z8_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100011u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
