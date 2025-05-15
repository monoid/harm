/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movt_r_zt_ {
    #[inline]
    pub fn movt_r_zt_(
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000010011000u32 << 15u32
                | u32::from(off3.into()) << 12u32
                | 0b0011111u32 << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
