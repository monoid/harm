/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AUTIASPPC_only_dp_1src_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011100000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTIASPPC_only_dp_1src_imm";
    #[inline]
    pub const fn AUTIASPPC_only_dp_1src_imm(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110011100u32 << 21u32 | imm16.into_inner() << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
pub mod AUTIBSPPC_only_dp_1src_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTIBSPPC_only_dp_1src_imm";
    #[inline]
    pub const fn AUTIBSPPC_only_dp_1src_imm(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110011101u32 << 21u32 | imm16.into_inner() << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
