/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADDPT_64_addsub_pt {
    #[inline]
    pub fn ADDPT_64_addsub_pt(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(imm3.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUBPT_64_addsub_pt {
    #[inline]
    pub fn SUBPT_64_addsub_pt(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(imm3.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
