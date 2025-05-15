/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod WFET_only_systeminstrswithreg {
    #[inline]
    pub fn WFET_only_systeminstrswithreg(
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010000001100010000000u32 << 5u32 | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod WFIT_only_systeminstrswithreg {
    #[inline]
    pub fn WFIT_only_systeminstrswithreg(
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010000001100010000001u32 << 5u32 | u32::from(Rd.into()) << 0u32,
        )
    }
}
