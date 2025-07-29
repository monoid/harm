/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MSR_SI_pstate {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111110001111000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000000100000000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MSR_SI_pstate";
    #[inline]
    pub const fn MSR_SI_pstate(
        op1: ::aarchmrs_types::BitValue<3>,
        CRm: ::aarchmrs_types::BitValue<4>,
        op2: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101010100000u32 << 19u32
                | op1.into_inner() << 16u32
                | 0b0100u32 << 12u32
                | CRm.into_inner() << 8u32
                | op2.into_inner() << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod CFINV_M_pstate {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000000100000000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFINV_M_pstate";
    #[inline]
    pub const fn CFINV_M_pstate() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000000100000000011111u32 << 0u32)
    }
}
pub mod XAFLAG_M_pstate {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000000100000000111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "XAFLAG_M_pstate";
    #[inline]
    pub const fn XAFLAG_M_pstate() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000000100000000111111u32 << 0u32)
    }
}
pub mod AXFLAG_M_pstate {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000000100000001011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AXFLAG_M_pstate";
    #[inline]
    pub const fn AXFLAG_M_pstate() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000000100000001011111u32 << 0u32)
    }
}
