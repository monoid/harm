/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptrue_pn_i_ {
    #[inline]
    pub fn ptrue_pn_i_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        PNd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1000000111100000010u32 << 3u32
                | u32::from(PNd.into()) << 0u32,
        )
    }
}
