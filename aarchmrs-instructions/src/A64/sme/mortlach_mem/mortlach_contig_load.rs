/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_za_p_rrr_ {
    #[inline]
    pub fn ld1b_za_p_rrr_(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        off4: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100000000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(off4.into()) << 0u32,
        )
    }
}
pub mod ld1h_za_p_rrr_ {
    #[inline]
    pub fn ld1h_za_p_rrr_(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAt: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100000010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(ZAt.into()) << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod ld1w_za_p_rrr_ {
    #[inline]
    pub fn ld1w_za_p_rrr_(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAt: impl Into<::aarchmrs_types::BitValue<2>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100000100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(ZAt.into()) << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod ld1d_za_p_rrr_ {
    #[inline]
    pub fn ld1d_za_p_rrr_(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        V: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ZAt: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100000110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(V.into()) << 15u32
                | u32::from(Rs.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(ZAt.into()) << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
