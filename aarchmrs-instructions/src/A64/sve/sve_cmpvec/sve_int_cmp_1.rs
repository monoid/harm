/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmpge_p_p_zw_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100000000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmpge_p_p_zw_";
    #[inline]
    pub const fn cmpge_p_p_zw_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmphs_p_p_zw_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmphs_p_p_zw_";
    #[inline]
    pub const fn cmphs_p_p_zw_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmpgt_p_p_zw_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100000000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmpgt_p_p_zw_";
    #[inline]
    pub const fn cmpgt_p_p_zw_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmphi_p_p_zw_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmphi_p_p_zw_";
    #[inline]
    pub const fn cmphi_p_p_zw_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmplt_p_p_zw_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100000000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmplt_p_p_zw_";
    #[inline]
    pub const fn cmplt_p_p_zw_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmplo_p_p_zw_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmplo_p_p_zw_";
    #[inline]
    pub const fn cmplo_p_p_zw_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmple_p_p_zw_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100000000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmple_p_p_zw_";
    #[inline]
    pub const fn cmple_p_p_zw_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod cmpls_p_p_zw_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100100000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmpls_p_p_zw_";
    #[inline]
    pub const fn cmpls_p_p_zw_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        lt: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ne: ::aarchmrs_types::BitValue<1>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | lt.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | ne.into_inner() << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
