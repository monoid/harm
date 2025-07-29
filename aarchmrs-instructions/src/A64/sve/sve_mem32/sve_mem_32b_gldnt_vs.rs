/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldnt1sb_z_p_ar_s_x32_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnt1sb_z_p_ar_s_x32_unscaled";
    #[inline]
    pub const fn ldnt1sb_z_p_ar_s_x32_unscaled(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnt1sh_z_p_ar_s_x32_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100100000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnt1sh_z_p_ar_s_x32_unscaled";
    #[inline]
    pub const fn ldnt1sh_z_p_ar_s_x32_unscaled(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100100u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnt1w_z_p_ar_s_x32_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000101000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnt1w_z_p_ar_s_x32_unscaled";
    #[inline]
    pub const fn ldnt1w_z_p_ar_s_x32_unscaled(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000101000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnt1b_z_p_ar_s_x32_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnt1b_z_p_ar_s_x32_unscaled";
    #[inline]
    pub const fn ldnt1b_z_p_ar_s_x32_unscaled(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnt1h_z_p_ar_s_x32_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100100000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnt1h_z_p_ar_s_x32_unscaled";
    #[inline]
    pub const fn ldnt1h_z_p_ar_s_x32_unscaled(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100100u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
