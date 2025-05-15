/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cpy_z_p_v_ {
    #[inline]
    pub fn cpy_z_p_v_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Vn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b100000100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Vn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
