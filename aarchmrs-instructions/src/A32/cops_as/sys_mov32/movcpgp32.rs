/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MCR_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111000100000000111000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000000111000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MCR_A1";
    #[inline]
    pub const fn MCR_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        opc1: ::aarchmrs_types::BitValue<3>,
        CRn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        cp15: ::aarchmrs_types::BitValue<1>,
        opc2: ::aarchmrs_types::BitValue<3>,
        CRm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1110u32 << 24u32
                | opc1.into_inner() << 21u32
                | 0b0u32 << 20u32
                | CRn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111u32 << 9u32
                | cp15.into_inner() << 8u32
                | opc2.into_inner() << 5u32
                | 0b1u32 << 4u32
                | CRm.into_inner() << 0u32,
        )
    }
}
pub mod MRC_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111000100000000111000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000100000000111000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MRC_A1";
    #[inline]
    pub const fn MRC_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        opc1: ::aarchmrs_types::BitValue<3>,
        CRn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        cp15: ::aarchmrs_types::BitValue<1>,
        opc2: ::aarchmrs_types::BitValue<3>,
        CRm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1110u32 << 24u32
                | opc1.into_inner() << 21u32
                | 0b1u32 << 20u32
                | CRn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111u32 << 9u32
                | cp15.into_inner() << 8u32
                | opc2.into_inner() << 5u32
                | 0b1u32 << 4u32
                | CRm.into_inner() << 0u32,
        )
    }
}
