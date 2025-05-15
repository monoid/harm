/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zero_za_i_ {
    #[inline]
    pub fn zero_za_i_(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000000000100000000000u32 << 8u32 | u32::from(imm8.into()) << 0u32,
        )
    }
}
