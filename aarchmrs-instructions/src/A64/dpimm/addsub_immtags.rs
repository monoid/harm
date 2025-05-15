/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADDG_64_addsub_immtags {
    #[inline]
    pub fn ADDG_64_addsub_immtags(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001000110u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm4.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUBG_64_addsub_immtags {
    #[inline]
    pub fn SUBG_64_addsub_immtags(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101000110u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm4.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
