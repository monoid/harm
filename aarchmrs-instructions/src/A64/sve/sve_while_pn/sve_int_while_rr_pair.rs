/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_pp_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000010001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000101000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilege_pp_rr_";
    #[inline]
    pub const fn whilege_pp_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b010100u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod whilehs_pp_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000010001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000101100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilehs_pp_rr_";
    #[inline]
    pub const fn whilehs_pp_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b010110u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod whilegt_pp_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000010001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000101000000010001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilegt_pp_rr_";
    #[inline]
    pub const fn whilegt_pp_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b010100u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod whilehi_pp_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000010001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000101100000010001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilehi_pp_rr_";
    #[inline]
    pub const fn whilehi_pp_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b010110u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod whilelt_pp_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000010001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000101010000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilelt_pp_rr_";
    #[inline]
    pub const fn whilelt_pp_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod whilelo_pp_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000010001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000101110000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilelo_pp_rr_";
    #[inline]
    pub const fn whilelo_pp_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b010111u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod whilele_pp_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000010001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000101010000010001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilele_pp_rr_";
    #[inline]
    pub const fn whilele_pp_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod whilels_pp_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000010001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000101110000010001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilels_pp_rr_";
    #[inline]
    pub const fn whilels_pp_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b010111u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
