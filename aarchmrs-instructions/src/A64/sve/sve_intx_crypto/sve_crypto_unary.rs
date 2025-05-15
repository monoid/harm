/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod aesmc_z_z_ {
    #[inline]
    pub fn aesmc_z_z_(
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010010000011100000000u32 << 5u32 | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod aesimc_z_z_ {
    #[inline]
    pub fn aesimc_z_z_(
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010010000011100100000u32 << 5u32 | u32::from(Zdn.into()) << 0u32,
        )
    }
}
