/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod addvl_r_ri_ {
    #[inline]
    pub fn addvl_r_ri_(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100001u32 << 21u32
                | u32::from(Rn.into()) << 16u32
                | 0b01010u32 << 11u32
                | u32::from(imm6.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod addpl_r_ri_ {
    #[inline]
    pub fn addpl_r_ri_(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100011u32 << 21u32
                | u32::from(Rn.into()) << 16u32
                | 0b01010u32 << 11u32
                | u32::from(imm6.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
