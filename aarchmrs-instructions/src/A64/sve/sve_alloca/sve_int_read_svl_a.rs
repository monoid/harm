/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod rdsvl_r_i_ {
    #[inline]
    pub fn rdsvl_r_i_(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001011111101011u32 << 11u32
                | u32::from(imm6.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
