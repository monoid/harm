/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RMIF_only_rmif {
    #[inline]
    pub fn RMIF_only_rmif(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        mask: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111010000u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b00001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(mask.into()) << 0u32,
        )
    }
}
