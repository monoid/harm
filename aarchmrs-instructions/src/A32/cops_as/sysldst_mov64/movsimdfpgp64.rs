/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VMOV_toss_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010000000000101000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_toss_A1";
    #[inline]
    pub const fn VMOV_toss_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11000100u32 << 20u32
                | Rt2.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b101000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_ss_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010100000000101000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_ss_A1";
    #[inline]
    pub const fn VMOV_ss_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11000101u32 << 20u32
                | Rt2.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b101000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_tod_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010000000000101100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_tod_A1";
    #[inline]
    pub const fn VMOV_tod_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11000100u32 << 20u32
                | Rt2.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b101100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_d_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010100000000101100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_d_A1";
    #[inline]
    pub const fn VMOV_d_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11000101u32 << 20u32
                | Rt2.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b101100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
