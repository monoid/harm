/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldr_p_bi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001110000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000101100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldr_p_bi_";
    #[inline]
    pub const fn ldr_p_bi_(
        imm9h: ::aarchmrs_types::BitValue<6>,
        imm9l: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010110u32 << 22u32
                | imm9h.into_inner() << 16u32
                | 0b000u32 << 13u32
                | imm9l.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pt.into_inner() << 0u32,
        )
    }
}
