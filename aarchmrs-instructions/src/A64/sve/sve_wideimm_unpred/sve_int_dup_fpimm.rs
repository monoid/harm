/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fdup_z_i_ {
    #[inline]
    pub fn fdup_z_i_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b111001110u32 << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
