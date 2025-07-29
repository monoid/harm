/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_p_p_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001110110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilege_p_p_rr_";
    #[inline]
    pub const fn whilege_p_p_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        sf: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | sf.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod whilehs_p_p_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001110110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilehs_p_p_rr_";
    #[inline]
    pub const fn whilehs_p_p_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        sf: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | sf.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod whilegt_p_p_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001110110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilegt_p_p_rr_";
    #[inline]
    pub const fn whilegt_p_p_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        sf: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | sf.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod whilehi_p_p_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001110110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilehi_p_p_rr_";
    #[inline]
    pub const fn whilehi_p_p_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        sf: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | sf.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod whilelt_p_p_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001110110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilelt_p_p_rr_";
    #[inline]
    pub const fn whilelt_p_p_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        sf: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | sf.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod whilelo_p_p_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001110110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilelo_p_p_rr_";
    #[inline]
    pub const fn whilelo_p_p_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        sf: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | sf.into_inner() << 12u32
                | 0b11u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod whilele_p_p_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001110110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000000010000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilele_p_p_rr_";
    #[inline]
    pub const fn whilele_p_p_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        sf: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | sf.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod whilels_p_p_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001110110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "whilels_p_p_rr_";
    #[inline]
    pub const fn whilels_p_p_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        sf: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | sf.into_inner() << 12u32
                | 0b11u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
