/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cntp_r_p_p_ {
    #[inline]
    pub fn cntp_r_p_p_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10000010u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod firstp_r_p_p_ {
    #[inline]
    pub fn firstp_r_p_p_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10000110u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod lastp_r_p_p_ {
    #[inline]
    pub fn lastp_r_p_p_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10001010u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
