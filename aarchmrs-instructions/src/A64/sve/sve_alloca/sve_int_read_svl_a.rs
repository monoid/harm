/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod rdsvl_r_i_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100101111110101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "rdsvl_r_i_";
    #[inline]
    pub const fn rdsvl_r_i_(
        imm6: ::aarchmrs_types::BitValue<6>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001001011111101011u32 << 11u32
                | imm6.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
