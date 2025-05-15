/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mul_z_zi_ {
    #[inline]
    pub fn mul_z_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b110000110u32 << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
