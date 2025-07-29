/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOV_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_r_T2";
    #[inline]
    pub const fn MOV_r_T2(
        op: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000u32 << 13u32
                | op.into_inner() << 11u32
                | imm5.into_inner() << 6u32
                | Rm.into_inner() << 3u32
                | Rd.into_inner() << 0u32,
        )
    }
}
