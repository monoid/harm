/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cntp_r_pn_ {
    #[inline]
    pub fn cntp_r_pn_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        vl: impl Into<::aarchmrs_types::BitValue<1>>,
        PNn: impl Into<::aarchmrs_types::BitValue<4>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10000010000u32 << 11u32
                | u32::from(vl.into()) << 10u32
                | 0b1u32 << 9u32
                | u32::from(PNn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
