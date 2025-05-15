/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldr_z_bi_ {
    #[inline]
    pub fn ldr_z_bi_(
        imm9h: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9l: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010110u32 << 22u32
                | u32::from(imm9h.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(imm9l.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
