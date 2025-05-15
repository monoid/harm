/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlall_za32_z8z8i_1 {
    #[inline]
    pub fn fmlall_za32_z8z8i_1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        i4h: impl Into<::aarchmrs_types::BitValue<1>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i4l: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010100u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(i4h.into()) << 15u32
                | u32::from(Rv.into()) << 13u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b000u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
