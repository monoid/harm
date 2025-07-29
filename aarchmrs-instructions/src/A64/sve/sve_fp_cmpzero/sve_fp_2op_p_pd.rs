/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcmge_p_p_z0_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000100000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcmge_p_p_z0_";
    #[inline]
    pub const fn fcmge_p_p_z0_(
        size: ::aarchmrs_types::BitValue<2>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b01000u32 << 17u32
                | lt.into_inner() << 16u32
                | 0b001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod fcmeq_p_p_z0_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000100100010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcmeq_p_p_z0_";
    #[inline]
    pub const fn fcmeq_p_p_z0_(
        size: ::aarchmrs_types::BitValue<2>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b01001u32 << 17u32
                | lt.into_inner() << 16u32
                | 0b001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod fcmgt_p_p_z0_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000100000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcmgt_p_p_z0_";
    #[inline]
    pub const fn fcmgt_p_p_z0_(
        size: ::aarchmrs_types::BitValue<2>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b01000u32 << 17u32
                | lt.into_inner() << 16u32
                | 0b001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod fcmlt_p_p_z0_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000100000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcmlt_p_p_z0_";
    #[inline]
    pub const fn fcmlt_p_p_z0_(
        size: ::aarchmrs_types::BitValue<2>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b01000u32 << 17u32
                | lt.into_inner() << 16u32
                | 0b001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod fcmne_p_p_z0_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000100100010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcmne_p_p_z0_";
    #[inline]
    pub const fn fcmne_p_p_z0_(
        size: ::aarchmrs_types::BitValue<2>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b01001u32 << 17u32
                | lt.into_inner() << 16u32
                | 0b001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod fcmle_p_p_z0_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000100000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcmle_p_p_z0_";
    #[inline]
    pub const fn fcmle_p_p_z0_(
        size: ::aarchmrs_types::BitValue<2>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b01000u32 << 17u32
                | lt.into_inner() << 16u32
                | 0b001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
