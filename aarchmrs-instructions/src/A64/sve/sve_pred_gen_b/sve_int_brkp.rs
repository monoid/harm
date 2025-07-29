/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod brkpa_p_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "brkpa_p_p_pp_";
    #[inline]
    pub const fn brkpa_p_p_pp_(
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010000u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod brkpas_p_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101010000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "brkpas_p_p_pp_";
    #[inline]
    pub const fn brkpas_p_p_pp_(
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010100u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod brkpb_p_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000001100000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "brkpb_p_p_pp_";
    #[inline]
    pub const fn brkpb_p_p_pp_(
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010000u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod brkpbs_p_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101010000001100000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "brkpbs_p_p_pp_";
    #[inline]
    pub const fn brkpbs_p_p_pp_(
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010100u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
