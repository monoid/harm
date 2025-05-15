/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ext_z_zi_des {
    #[inline]
    pub fn ext_z_zi_des(
        imm8h: impl Into<::aarchmrs_types::BitValue<5>>,
        imm8l: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101001u32 << 21u32
                | u32::from(imm8h.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(imm8l.into()) << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
