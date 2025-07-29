/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmphs_p_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000010000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmphs_p_p_zi_";
    #[inline]
    pub const fn cmphs_p_p_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm7: ::aarchmrs_types::BitValue<7>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm7.into_inner() << 14u32
                | 0b0u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmphi_p_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000010000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100001000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmphi_p_p_zi_";
    #[inline]
    pub const fn cmphi_p_p_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm7: ::aarchmrs_types::BitValue<7>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm7.into_inner() << 14u32
                | 0b0u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmplo_p_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000010000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmplo_p_p_zi_";
    #[inline]
    pub const fn cmplo_p_p_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm7: ::aarchmrs_types::BitValue<7>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm7.into_inner() << 14u32
                | 0b1u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmpls_p_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000010000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100001000000010000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmpls_p_p_zi_";
    #[inline]
    pub const fn cmpls_p_p_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm7: ::aarchmrs_types::BitValue<7>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm7.into_inner() << 14u32
                | 0b1u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
