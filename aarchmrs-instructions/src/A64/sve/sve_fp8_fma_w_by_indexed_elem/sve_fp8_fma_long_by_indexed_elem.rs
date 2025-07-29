/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlalb_z_z8z8z8i_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100001000000101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlalb_z_z8z8z8i_";
    #[inline]
    pub const fn fmlalb_z_z8z8z8i_(
        i4h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i4l: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | i4h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b0101u32 << 12u32
                | i4l.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod fmlalt_z_z8z8z8i_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100101000000101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlalt_z_z8z8z8i_";
    #[inline]
    pub const fn fmlalt_z_z8z8z8i_(
        i4h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i4l: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100101u32 << 21u32
                | i4h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b0101u32 << 12u32
                | i4l.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
