/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod insr_z_r_ {
    #[inline]
    pub fn insr_z_r_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b100100001110u32 << 10u32
                | u32::from(Rm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
