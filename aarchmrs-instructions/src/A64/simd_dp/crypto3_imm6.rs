/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod XAR_VVV2_crypto3_imm6 {
    #[inline]
    pub fn XAR_VVV2_crypto3_imm6(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
