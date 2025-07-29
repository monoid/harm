/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCCMP_S_floatccmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCCMP_S_floatccmp";
    #[inline]
    pub const fn FCCMP_S_floatccmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        nzcv: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | nzcv.into_inner() << 0u32,
        )
    }
}
pub mod FCCMPE_S_floatccmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000000000010000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCCMPE_S_floatccmp";
    #[inline]
    pub const fn FCCMPE_S_floatccmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        nzcv: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | nzcv.into_inner() << 0u32,
        )
    }
}
pub mod FCCMP_D_floatccmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCCMP_D_floatccmp";
    #[inline]
    pub const fn FCCMP_D_floatccmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        nzcv: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | nzcv.into_inner() << 0u32,
        )
    }
}
pub mod FCCMPE_D_floatccmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000000000010000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCCMPE_D_floatccmp";
    #[inline]
    pub const fn FCCMPE_D_floatccmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        nzcv: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | nzcv.into_inner() << 0u32,
        )
    }
}
pub mod FCCMP_H_floatccmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCCMP_H_floatccmp";
    #[inline]
    pub const fn FCCMP_H_floatccmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        nzcv: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | nzcv.into_inner() << 0u32,
        )
    }
}
pub mod FCCMPE_H_floatccmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000000000010000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCCMPE_H_floatccmp";
    #[inline]
    pub const fn FCCMPE_H_floatccmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        nzcv: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | nzcv.into_inner() << 0u32,
        )
    }
}
