/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VMOV_rs_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111100100000000111100011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000000101100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_rs_A1";
    #[inline]
    pub const fn VMOV_rs_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        opc1: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        opc2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11100u32 << 23u32
                | opc1.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Vd.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | D.into_inner() << 7u32
                | opc2.into_inner() << 5u32
                | 0b10000u32 << 0u32,
        )
    }
}
pub mod VMOV_sr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111000100000000111100011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000100000000101100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_sr_A1";
    #[inline]
    pub const fn VMOV_sr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        opc1: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        opc2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1110u32 << 24u32
                | U.into_inner() << 23u32
                | opc1.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Vn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | N.into_inner() << 7u32
                | opc2.into_inner() << 5u32
                | 0b10000u32 << 0u32,
        )
    }
}
pub mod VDUP_r_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111101011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101000000000101100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VDUP_r_A1_Q";
    #[inline]
    pub const fn VDUP_r_A1_Q(
        cond: ::aarchmrs_types::BitValue<4>,
        B: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        E: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | B.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Vd.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | D.into_inner() << 7u32
                | 0b0u32 << 6u32
                | E.into_inner() << 5u32
                | 0b10000u32 << 0u32,
        )
    }
}
pub mod VDUP_r_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111101011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110100000000000101100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VDUP_r_A1_D";
    #[inline]
    pub const fn VDUP_r_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        B: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        E: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | B.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Vd.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | D.into_inner() << 7u32
                | 0b0u32 << 6u32
                | E.into_inner() << 5u32
                | 0b10000u32 << 0u32,
        )
    }
}
