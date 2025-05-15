/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movt_zt_z_ {
    #[inline]
    pub fn movt_zt_z_(
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000000100111100u32 << 14u32
                | u32::from(off2.into()) << 12u32
                | 0b0011111u32 << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
