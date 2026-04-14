/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADR_only_pcreladdr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10011111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00010000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADR_only_pcreladdr";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rd_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rd_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_immhi_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_immhi_WIDTH: u32 = 19u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_immlo_OFFSET: u32 = 29u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_immlo_WIDTH: u32 = 2u32;
    #[inline]
    pub const fn ADR_only_pcreladdr(
        immlo: ::aarchmrs_types::BitValue<2>,
        immhi: ::aarchmrs_types::BitValue<19>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | immlo.into_inner() << 29u32
                | 0b10000u32 << 24u32
                | immhi.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ADRP_only_pcreladdr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10011111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10010000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADRP_only_pcreladdr";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rd_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rd_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_immhi_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_immhi_WIDTH: u32 = 19u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_immlo_OFFSET: u32 = 29u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_immlo_WIDTH: u32 = 2u32;
    #[inline]
    pub const fn ADRP_only_pcreladdr(
        immlo: ::aarchmrs_types::BitValue<2>,
        immhi: ::aarchmrs_types::BitValue<19>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1u32 << 31u32
                | immlo.into_inner() << 29u32
                | 0b10000u32 << 24u32
                | immhi.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
