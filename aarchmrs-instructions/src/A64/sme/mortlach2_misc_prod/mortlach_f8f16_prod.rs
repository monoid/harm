/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmopa_za16_pp_z8z8_8 {
    #[inline]
    pub fn fmopa_za16_pp_z8z8_8(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pm: impl Into<::aarchmrs_types::BitValue<3>>,
        Pn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000101u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(Pm.into()) << 13u32
                | u32::from(Pn.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b0100u32 << 1u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
