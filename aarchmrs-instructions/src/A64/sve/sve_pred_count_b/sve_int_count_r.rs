/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incp_r_p_r_ {
    #[inline]
    pub fn incp_r_p_r_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1011001000100u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod decp_r_p_r_ {
    #[inline]
    pub fn decp_r_p_r_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1011011000100u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
