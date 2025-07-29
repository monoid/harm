/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STR_r_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STR_r_A1_off";
    #[inline]
    pub const fn STR_r_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STR_r_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STR_r_A1_post";
    #[inline]
    pub const fn STR_r_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STR_r_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STR_r_A1_pre";
    #[inline]
    pub const fn STR_r_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDR_r_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000111000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_r_A1_off";
    #[inline]
    pub const fn LDR_r_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDR_r_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_r_A1_post";
    #[inline]
    pub const fn LDR_r_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDR_r_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000111001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_r_A1_pre";
    #[inline]
    pub const fn LDR_r_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRT_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRT_A2";
    #[inline]
    pub const fn STRT_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110u32 << 24u32
                | U.into_inner() << 23u32
                | 0b010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRT_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRT_A2";
    #[inline]
    pub const fn LDRT_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRB_r_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000111010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRB_r_A1_off";
    #[inline]
    pub const fn STRB_r_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRB_r_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRB_r_A1_post";
    #[inline]
    pub const fn STRB_r_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110u32 << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRB_r_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000111011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRB_r_A1_pre";
    #[inline]
    pub const fn STRB_r_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_r_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000111010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_r_A1_off";
    #[inline]
    pub const fn LDRB_r_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_r_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_r_A1_post";
    #[inline]
    pub const fn LDRB_r_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_r_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_r_A1_pre";
    #[inline]
    pub const fn LDRB_r_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0111u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRBT_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRBT_A2";
    #[inline]
    pub const fn STRBT_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110u32 << 24u32
                | U.into_inner() << 23u32
                | 0b110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRBT_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRBT_A2";
    #[inline]
    pub const fn LDRBT_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | stype.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
