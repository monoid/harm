/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SYSP_CR_syspairinstrs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101010010000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SYSP_CR_syspairinstrs";
    #[inline]
    pub const fn SYSP_CR_syspairinstrs(
        op1: ::aarchmrs_types::BitValue<3>,
        CRn: ::aarchmrs_types::BitValue<4>,
        CRm: ::aarchmrs_types::BitValue<4>,
        op2: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101010101001u32 << 19u32
                | op1.into_inner() << 16u32
                | CRn.into_inner() << 12u32
                | CRm.into_inner() << 8u32
                | op2.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
