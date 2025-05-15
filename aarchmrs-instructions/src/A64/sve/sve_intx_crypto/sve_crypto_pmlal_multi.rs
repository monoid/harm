/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pmlal_mz_zzzw_1x2 {
    #[inline]
    pub fn pmlal_mz_zzzw_1x2(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
