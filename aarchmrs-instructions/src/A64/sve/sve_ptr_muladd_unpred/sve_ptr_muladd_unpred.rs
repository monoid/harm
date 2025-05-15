/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mlapt_z_zzz_ {
    #[inline]
    pub fn mlapt_z_zzz_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100110u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b110100u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod madpt_z_zzz_ {
    #[inline]
    pub fn madpt_z_zzz_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Za: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100110u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b110110u32 << 10u32
                | u32::from(Za.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
