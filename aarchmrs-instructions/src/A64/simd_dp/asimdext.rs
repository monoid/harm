/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod EXT_asimdext_only {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EXT_asimdext_only";
    #[inline]
    pub const fn EXT_asimdext_only(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm4.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
