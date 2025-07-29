/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MSRR_SR_systemmovepr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MSRR_SR_systemmovepr";
    #[inline]
    pub const fn MSRR_SR_systemmovepr(
        o0: ::aarchmrs_types::BitValue<1>,
        op1: ::aarchmrs_types::BitValue<3>,
        CRn: ::aarchmrs_types::BitValue<4>,
        CRm: ::aarchmrs_types::BitValue<4>,
        op2: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010101u32 << 20u32
                | o0.into_inner() << 19u32
                | op1.into_inner() << 16u32
                | CRn.into_inner() << 12u32
                | CRm.into_inner() << 8u32
                | op2.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod MRRS_RS_systemmovepr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MRRS_RS_systemmovepr";
    #[inline]
    pub const fn MRRS_RS_systemmovepr(
        o0: ::aarchmrs_types::BitValue<1>,
        op1: ::aarchmrs_types::BitValue<3>,
        CRn: ::aarchmrs_types::BitValue<4>,
        CRm: ::aarchmrs_types::BitValue<4>,
        op2: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010111u32 << 20u32
                | o0.into_inner() << 19u32
                | op1.into_inner() << 16u32
                | CRn.into_inner() << 12u32
                | CRm.into_inner() << 8u32
                | op2.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
