/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dupm_z_i_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "dupm_z_i_";
    #[inline]
    pub const fn dupm_z_i_(
        imm13: ::aarchmrs_types::BitValue<13>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101110000u32 << 18u32 | imm13.into_inner() << 5u32 | Zd.into_inner() << 0u32,
        )
    }
}
