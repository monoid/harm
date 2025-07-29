/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincb_r_rs_sx {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincb_r_rs_sx";
    #[inline]
    pub const fn sqincb_r_rs_sx(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111100u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincb_r_rs_uw {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincb_r_rs_uw";
    #[inline]
    pub const fn uqincb_r_rs_uw(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecb_r_rs_sx {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecb_r_rs_sx";
    #[inline]
    pub const fn sqdecb_r_rs_sx(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111110u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecb_r_rs_uw {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecb_r_rs_uw";
    #[inline]
    pub const fn uqdecb_r_rs_uw(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqincb_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincb_r_rs_x";
    #[inline]
    pub const fn sqincb_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11110u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecb_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecb_r_rs_x";
    #[inline]
    pub const fn sqdecb_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11111u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqinch_r_rs_sx {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqinch_r_rs_sx";
    #[inline]
    pub const fn sqinch_r_rs_sx(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111100u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqinch_r_rs_uw {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqinch_r_rs_uw";
    #[inline]
    pub const fn uqinch_r_rs_uw(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdech_r_rs_sx {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdech_r_rs_sx";
    #[inline]
    pub const fn sqdech_r_rs_sx(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111110u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdech_r_rs_uw {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdech_r_rs_uw";
    #[inline]
    pub const fn uqdech_r_rs_uw(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqinch_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqinch_r_rs_x";
    #[inline]
    pub const fn sqinch_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11110u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdech_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdech_r_rs_x";
    #[inline]
    pub const fn sqdech_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11111u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqincw_r_rs_sx {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincw_r_rs_sx";
    #[inline]
    pub const fn sqincw_r_rs_sx(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111100u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincw_r_rs_uw {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincw_r_rs_uw";
    #[inline]
    pub const fn uqincw_r_rs_uw(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecw_r_rs_sx {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecw_r_rs_sx";
    #[inline]
    pub const fn sqdecw_r_rs_sx(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111110u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecw_r_rs_uw {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecw_r_rs_uw";
    #[inline]
    pub const fn uqdecw_r_rs_uw(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqincw_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincw_r_rs_x";
    #[inline]
    pub const fn sqincw_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11110u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecw_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecw_r_rs_x";
    #[inline]
    pub const fn sqdecw_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11111u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqincd_r_rs_sx {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincd_r_rs_sx";
    #[inline]
    pub const fn sqincd_r_rs_sx(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111100u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincd_r_rs_uw {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincd_r_rs_uw";
    #[inline]
    pub const fn uqincd_r_rs_uw(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecd_r_rs_sx {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecd_r_rs_sx";
    #[inline]
    pub const fn sqdecd_r_rs_sx(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111110u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecd_r_rs_uw {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecd_r_rs_uw";
    #[inline]
    pub const fn uqdecd_r_rs_uw(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqincd_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincd_r_rs_x";
    #[inline]
    pub const fn sqincd_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11110u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecd_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecd_r_rs_x";
    #[inline]
    pub const fn sqdecd_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11111u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincb_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincb_r_rs_x";
    #[inline]
    pub const fn uqincb_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11110u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecb_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecb_r_rs_x";
    #[inline]
    pub const fn uqdecb_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11111u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqinch_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqinch_r_rs_x";
    #[inline]
    pub const fn uqinch_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11110u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdech_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdech_r_rs_x";
    #[inline]
    pub const fn uqdech_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11111u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincw_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincw_r_rs_x";
    #[inline]
    pub const fn uqincw_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11110u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecw_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecw_r_rs_x";
    #[inline]
    pub const fn uqdecw_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11111u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincd_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincd_r_rs_x";
    #[inline]
    pub const fn uqincd_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11110u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecd_r_rs_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecd_r_rs_x";
    #[inline]
    pub const fn uqdecd_r_rs_x(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b11111u32 << 11u32
                | U.into_inner() << 10u32
                | pattern.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
