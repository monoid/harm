/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincp_z_p_z_ {
    #[inline]
    pub fn sqincp_z_p_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10100u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b1000000u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sqdecp_z_p_z_ {
    #[inline]
    pub fn sqdecp_z_p_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10101u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b1000000u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqincp_z_p_z_ {
    #[inline]
    pub fn uqincp_z_p_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10100u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b1000000u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqdecp_z_p_z_ {
    #[inline]
    pub fn uqdecp_z_p_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10101u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b1000000u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
