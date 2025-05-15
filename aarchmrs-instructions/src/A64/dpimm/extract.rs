/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod EXTR_32_extract {
    #[inline]
    pub fn EXTR_32_extract(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imms: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00010011100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod EXTR_64_extract {
    #[inline]
    pub fn EXTR_64_extract(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10010011110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
