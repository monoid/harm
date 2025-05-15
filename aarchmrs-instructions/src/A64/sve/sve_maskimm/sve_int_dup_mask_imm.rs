/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dupm_z_i_ {
    #[inline]
    pub fn dupm_z_i_(
        imm13: impl Into<::aarchmrs_types::BitValue<13>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101110000u32 << 18u32
                | u32::from(imm13.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
