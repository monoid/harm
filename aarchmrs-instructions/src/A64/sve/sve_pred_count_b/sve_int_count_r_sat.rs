/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincp_r_p_r_sx {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincp_r_p_r_sx";
    #[inline]
    pub const fn sqincp_r_p_r_sx(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1010001000100u32 << 9u32
                | Pm.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincp_r_p_r_uw {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincp_r_p_r_uw";
    #[inline]
    pub const fn uqincp_r_p_r_uw(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1010011000100u32 << 9u32
                | Pm.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecp_r_p_r_sx {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010101000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecp_r_p_r_sx";
    #[inline]
    pub const fn sqdecp_r_p_r_sx(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1010101000100u32 << 9u32
                | Pm.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecp_r_p_r_uw {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010111000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecp_r_p_r_uw";
    #[inline]
    pub const fn uqdecp_r_p_r_uw(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1010111000100u32 << 9u32
                | Pm.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqincp_r_p_r_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010001000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincp_r_p_r_x";
    #[inline]
    pub const fn sqincp_r_p_r_x(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1010001000110u32 << 9u32
                | Pm.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecp_r_p_r_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010101000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecp_r_p_r_x";
    #[inline]
    pub const fn sqdecp_r_p_r_x(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1010101000110u32 << 9u32
                | Pm.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincp_r_p_r_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010011000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincp_r_p_r_x";
    #[inline]
    pub const fn uqincp_r_p_r_x(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1010011000110u32 << 9u32
                | Pm.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecp_r_p_r_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010111000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecp_r_p_r_x";
    #[inline]
    pub const fn uqdecp_r_p_r_x(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1010111000110u32 << 9u32
                | Pm.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
