/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MSR_SR_systemmove {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MSR_SR_systemmove";
    #[inline]
    pub const fn MSR_SR_systemmove(
        o0: ::aarchmrs_types::BitValue<1>,
        op1: ::aarchmrs_types::BitValue<3>,
        CRn: ::aarchmrs_types::BitValue<4>,
        CRm: ::aarchmrs_types::BitValue<4>,
        op2: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010001u32 << 20u32
                | o0.into_inner() << 19u32
                | op1.into_inner() << 16u32
                | CRn.into_inner() << 12u32
                | CRm.into_inner() << 8u32
                | op2.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod MRS_RS_systemmove {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MRS_RS_systemmove";
    #[inline]
    pub const fn MRS_RS_systemmove(
        o0: ::aarchmrs_types::BitValue<1>,
        op1: ::aarchmrs_types::BitValue<3>,
        CRn: ::aarchmrs_types::BitValue<4>,
        CRm: ::aarchmrs_types::BitValue<4>,
        op2: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010011u32 << 20u32
                | o0.into_inner() << 19u32
                | op1.into_inner() << 16u32
                | CRn.into_inner() << 12u32
                | CRm.into_inner() << 8u32
                | op2.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
