/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod index_z_ir_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000000100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "index_z_ir_";
    #[inline]
    pub const fn index_z_ir_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b010010u32 << 10u32
                | imm5.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
