/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_z_zz_16 {
    #[inline]
    pub fn luti2_z_zz_16(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | u32::from(i3h.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(i3l.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
