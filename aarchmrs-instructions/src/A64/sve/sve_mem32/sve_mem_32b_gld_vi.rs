/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1sb_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1sb_z_p_ai_s";
    #[inline]
    pub const fn ld1sb_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100001u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1sh_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100101000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1sh_z_p_ai_s";
    #[inline]
    pub const fn ld1sh_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100101u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b100u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1w_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000101001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1w_z_p_ai_s";
    #[inline]
    pub const fn ld1w_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000101001u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1b_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1b_z_p_ai_s";
    #[inline]
    pub const fn ld1b_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100001u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1h_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100101000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1h_z_p_ai_s";
    #[inline]
    pub const fn ld1h_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100101u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1sb_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100001000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1sb_z_p_ai_s";
    #[inline]
    pub const fn ldff1sb_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100001u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1sh_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100101000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1sh_z_p_ai_s";
    #[inline]
    pub const fn ldff1sh_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100101u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1w_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000101001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1w_z_p_ai_s";
    #[inline]
    pub const fn ldff1w_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000101001u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1b_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1b_z_p_ai_s";
    #[inline]
    pub const fn ldff1b_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100001u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1h_z_p_ai_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1h_z_p_ai_s";
    #[inline]
    pub const fn ldff1h_z_p_ai_s(
        imm5: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000100101u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
