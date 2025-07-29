/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TBZ_only_testbranch {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b01111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00110110000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TBZ_only_testbranch";
    #[inline]
    pub const fn TBZ_only_testbranch(
        b5: ::aarchmrs_types::BitValue<1>,
        b40: ::aarchmrs_types::BitValue<5>,
        imm14: ::aarchmrs_types::BitValue<14>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            b5.into_inner() << 31u32
                | 0b0110110u32 << 24u32
                | b40.into_inner() << 19u32
                | imm14.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod TBNZ_only_testbranch {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b01111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00110111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TBNZ_only_testbranch";
    #[inline]
    pub const fn TBNZ_only_testbranch(
        b5: ::aarchmrs_types::BitValue<1>,
        b40: ::aarchmrs_types::BitValue<5>,
        imm14: ::aarchmrs_types::BitValue<14>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            b5.into_inner() << 31u32
                | 0b0110111u32 << 24u32
                | b40.into_inner() << 19u32
                | imm14.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
