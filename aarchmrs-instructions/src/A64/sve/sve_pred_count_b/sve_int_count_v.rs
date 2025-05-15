/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incp_z_p_z_ {
    #[inline]
    pub fn incp_z_p_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1011001000000u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod decp_z_p_z_ {
    #[inline]
    pub fn decp_z_p_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1011011000000u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
