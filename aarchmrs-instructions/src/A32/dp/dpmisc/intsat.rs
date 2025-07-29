/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod QADD_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000000000000000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QADD_A1";
    #[inline]
    pub const fn QADD_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | 0b00000101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QSUB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001000000000000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QSUB_A1";
    #[inline]
    pub const fn QSUB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | 0b00000101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QDADD_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010000000000000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QDADD_A1";
    #[inline]
    pub const fn QDADD_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | 0b00000101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QDSUB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011000000000000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QDSUB_A1";
    #[inline]
    pub const fn QDSUB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | 0b00000101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
