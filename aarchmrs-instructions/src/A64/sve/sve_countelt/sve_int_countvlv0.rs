/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqinch_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqinch_z_zs_";
    #[inline]
    pub const fn sqinch_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001000110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdech_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011000001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdech_z_zs_";
    #[inline]
    pub const fn sqdech_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001000110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110010u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqinch_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqinch_z_zs_";
    #[inline]
    pub const fn uqinch_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001000110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110001u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdech_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdech_z_zs_";
    #[inline]
    pub const fn uqdech_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001000110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110011u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sqincw_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100101000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincw_z_zs_";
    #[inline]
    pub const fn sqincw_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001010u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecw_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100101000001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecw_z_zs_";
    #[inline]
    pub const fn sqdecw_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001010u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110010u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincw_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100101000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincw_z_zs_";
    #[inline]
    pub const fn uqincw_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001010u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110001u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecw_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100101000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecw_z_zs_";
    #[inline]
    pub const fn uqdecw_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001010u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110011u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sqincd_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincd_z_zs_";
    #[inline]
    pub const fn sqincd_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecd_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100111000001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecd_z_zs_";
    #[inline]
    pub const fn sqdecd_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110010u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincd_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100111000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincd_z_zs_";
    #[inline]
    pub const fn uqincd_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110001u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecd_z_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100111000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecd_z_zs_";
    #[inline]
    pub const fn uqdecd_z_zs_(
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110011u32 << 10u32
                | pattern.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
