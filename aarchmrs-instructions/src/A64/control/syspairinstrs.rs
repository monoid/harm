/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SYSP_CR_syspairinstrs {
    #[inline]
    pub fn SYSP_CR_syspairinstrs(
        op1: impl Into<::aarchmrs_types::BitValue<3>>,
        CRn: impl Into<::aarchmrs_types::BitValue<4>>,
        CRm: impl Into<::aarchmrs_types::BitValue<4>>,
        op2: impl Into<::aarchmrs_types::BitValue<3>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101010101001u32 << 19u32
                | u32::from(op1.into()) << 16u32
                | u32::from(CRn.into()) << 12u32
                | u32::from(CRm.into()) << 8u32
                | u32::from(op2.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
