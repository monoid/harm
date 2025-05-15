/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod psel_p_ppi_ {
    #[inline]
    pub fn psel_p_ppi_(
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<3>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(i1.into()) << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 18u32
                | u32::from(Rv.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pn.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
