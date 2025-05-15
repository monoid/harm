/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincp_r_p_r_sx {
    #[inline]
    pub fn sqincp_r_p_r_sx(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1010001000100u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqincp_r_p_r_uw {
    #[inline]
    pub fn uqincp_r_p_r_uw(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1010011000100u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqdecp_r_p_r_sx {
    #[inline]
    pub fn sqdecp_r_p_r_sx(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1010101000100u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqdecp_r_p_r_uw {
    #[inline]
    pub fn uqdecp_r_p_r_uw(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1010111000100u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqincp_r_p_r_x {
    #[inline]
    pub fn sqincp_r_p_r_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10100u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b1000110u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqdecp_r_p_r_x {
    #[inline]
    pub fn sqdecp_r_p_r_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10101u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b1000110u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqincp_r_p_r_x {
    #[inline]
    pub fn uqincp_r_p_r_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10100u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b1000110u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqdecp_r_p_r_x {
    #[inline]
    pub fn uqdecp_r_p_r_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10101u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b1000110u32 << 9u32
                | u32::from(Pm.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
