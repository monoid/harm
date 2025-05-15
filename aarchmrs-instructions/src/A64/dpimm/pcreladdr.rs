/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADR_only_pcreladdr {
    #[inline]
    pub fn ADR_only_pcreladdr(
        immlo: impl Into<::aarchmrs_types::BitValue<2>>,
        immhi: impl Into<::aarchmrs_types::BitValue<19>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(immlo.into()) << 29u32
                | 0b10000u32 << 24u32
                | u32::from(immhi.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADRP_only_pcreladdr {
    #[inline]
    pub fn ADRP_only_pcreladdr(
        immlo: impl Into<::aarchmrs_types::BitValue<2>>,
        immhi: impl Into<::aarchmrs_types::BitValue<19>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1u32 << 31u32
                | u32::from(immlo.into()) << 29u32
                | 0b10000u32 << 24u32
                | u32::from(immhi.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
