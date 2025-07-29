/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod xar_z_zzi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000000011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "xar_z_zzi_";
    #[inline]
    pub const fn xar_z_zzi_(
        tszh: ::aarchmrs_types::BitValue<2>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b001101u32 << 10u32
                | Zm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
