/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TBZ_only_testbranch {
    #[inline]
    pub fn TBZ_only_testbranch(
        b5: impl Into<::aarchmrs_types::BitValue<1>>,
        b40: impl Into<::aarchmrs_types::BitValue<5>>,
        imm14: impl Into<::aarchmrs_types::BitValue<14>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(b5.into()) << 31u32
                | 0b0110110u32 << 24u32
                | u32::from(b40.into()) << 19u32
                | u32::from(imm14.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod TBNZ_only_testbranch {
    #[inline]
    pub fn TBNZ_only_testbranch(
        b5: impl Into<::aarchmrs_types::BitValue<1>>,
        b40: impl Into<::aarchmrs_types::BitValue<5>>,
        imm14: impl Into<::aarchmrs_types::BitValue<14>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(b5.into()) << 31u32
                | 0b0110111u32 << 24u32
                | u32::from(b40.into()) << 19u32
                | u32::from(imm14.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
