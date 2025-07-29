/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmpge_p_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmpge_p_p_zi_";
    #[inline]
    pub const fn cmpge_p_p_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b00u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmpeq_p_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmpeq_p_p_zi_";
    #[inline]
    pub const fn cmpeq_p_p_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmplt_p_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmplt_p_p_zi_";
    #[inline]
    pub const fn cmplt_p_p_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b00u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmpgt_p_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmpgt_p_p_zi_";
    #[inline]
    pub const fn cmpgt_p_p_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b00u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmpne_p_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmpne_p_p_zi_";
    #[inline]
    pub const fn cmpne_p_p_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmple_p_p_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmple_p_p_zi_";
    #[inline]
    pub const fn cmple_p_p_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b00u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
