/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod UDF_only_perm_undef {
    #[inline]
    pub fn UDF_only_perm_undef(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0000000000000000u32 << 16u32 | u32::from(imm16.into()) << 0u32,
        )
    }
}
