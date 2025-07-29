/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_pn_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001101110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000100000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilege_pn_rr_";
    #[inline]
    pub const fn whilege_pn_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        vl: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        PNd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | vl.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b10u32 << 3u32
                | PNd.into_inner() << 0u32,
        )
    }
}
pub mod whilehs_pn_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001101110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000100100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilehs_pn_rr_";
    #[inline]
    pub const fn whilehs_pn_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        vl: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        PNd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | vl.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b10u32 << 3u32
                | PNd.into_inner() << 0u32,
        )
    }
}
pub mod whilegt_pn_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001101110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000100000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilegt_pn_rr_";
    #[inline]
    pub const fn whilegt_pn_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        vl: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        PNd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | vl.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b11u32 << 3u32
                | PNd.into_inner() << 0u32,
        )
    }
}
pub mod whilehi_pn_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001101110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000100100000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilehi_pn_rr_";
    #[inline]
    pub const fn whilehi_pn_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        vl: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        PNd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | vl.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b11u32 << 3u32
                | PNd.into_inner() << 0u32,
        )
    }
}
pub mod whilelt_pn_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001101110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000100010000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilelt_pn_rr_";
    #[inline]
    pub const fn whilelt_pn_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        vl: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        PNd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | vl.into_inner() << 13u32
                | 0b001u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b10u32 << 3u32
                | PNd.into_inner() << 0u32,
        )
    }
}
pub mod whilelo_pn_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001101110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000100110000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilelo_pn_rr_";
    #[inline]
    pub const fn whilelo_pn_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        vl: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        PNd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | vl.into_inner() << 13u32
                | 0b011u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b10u32 << 3u32
                | PNd.into_inner() << 0u32,
        )
    }
}
pub mod whilele_pn_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001101110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000100010000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilele_pn_rr_";
    #[inline]
    pub const fn whilele_pn_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        vl: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        PNd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | vl.into_inner() << 13u32
                | 0b001u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b11u32 << 3u32
                | PNd.into_inner() << 0u32,
        )
    }
}
pub mod whilels_pn_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001101110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000100110000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilels_pn_rr_";
    #[inline]
    pub const fn whilels_pn_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        vl: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        PNd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | vl.into_inner() << 13u32
                | 0b011u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b11u32 << 3u32
                | PNd.into_inner() << 0u32,
        )
    }
}
