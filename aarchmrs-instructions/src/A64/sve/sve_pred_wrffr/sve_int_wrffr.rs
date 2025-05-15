/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod wrffr_f_p_ {
    #[inline]
    pub fn wrffr_f_p_(
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101001010001001000u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b00000u32 << 0u32,
        )
    }
}
