/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MCRR_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010000000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MCRR_A1";
    #[inline]
    pub const fn MCRR_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        cp15: ::aarchmrs_types::BitValue<1>,
        opc1: ::aarchmrs_types::BitValue<4>,
        CRm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11000100u32 << 20u32
                | Rt2.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111u32 << 9u32
                | cp15.into_inner() << 8u32
                | opc1.into_inner() << 4u32
                | CRm.into_inner() << 0u32,
        )
    }
}
pub mod MRRC_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MRRC_A1";
    #[inline]
    pub const fn MRRC_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        cp15: ::aarchmrs_types::BitValue<1>,
        opc1: ::aarchmrs_types::BitValue<4>,
        CRm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11000101u32 << 20u32
                | Rt2.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111u32 << 9u32
                | cp15.into_inner() << 8u32
                | opc1.into_inner() << 4u32
                | CRm.into_inner() << 0u32,
        )
    }
}
