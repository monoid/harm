/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_i_T1";
    #[inline]
    pub const fn ADD_i_T1(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001110u32 << 9u32
                | imm3.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SUB_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000001111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_i_T1";
    #[inline]
    pub const fn SUB_i_T1(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111u32 << 9u32
                | imm3.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rd.into_inner() << 0u32,
        )
    }
}
