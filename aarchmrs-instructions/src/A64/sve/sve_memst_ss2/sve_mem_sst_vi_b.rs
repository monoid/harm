/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100011000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1b_z_p_ai_s";
    #[inline]
    pub const fn st1b_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100100011u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st1h_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100111000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1h_z_p_ai_s";
    #[inline]
    pub const fn st1h_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100100111u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st1w_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101011000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1w_z_p_ai_s";
    #[inline]
    pub const fn st1w_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100101011u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
