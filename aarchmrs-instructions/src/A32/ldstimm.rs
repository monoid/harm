/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDR_l_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001110010111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_l_A1";
    #[inline]
    pub const fn LDR_l_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        P: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b010u32 << 25u32
                | P.into_inner() << 24u32
                | U.into_inner() << 23u32
                | 0b0u32 << 22u32
                | W.into_inner() << 21u32
                | 0b11111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_l_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001110010111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100010111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_l_A1";
    #[inline]
    pub const fn LDRB_l_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        P: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b010u32 << 25u32
                | P.into_inner() << 24u32
                | U.into_inner() << 23u32
                | 0b1u32 << 22u32
                | W.into_inner() << 21u32
                | 0b11111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod STR_i_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STR_i_A1_off";
    #[inline]
    pub const fn STR_i_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod STR_i_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STR_i_A1_post";
    #[inline]
    pub const fn STR_i_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod STR_i_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STR_i_A1_pre";
    #[inline]
    pub const fn STR_i_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDR_i_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_i_A1_off";
    #[inline]
    pub const fn LDR_i_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDR_i_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_i_A1_post";
    #[inline]
    pub const fn LDR_i_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDR_i_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_i_A1_pre";
    #[inline]
    pub const fn LDR_i_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod STRB_i_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRB_i_A1_off";
    #[inline]
    pub const fn STRB_i_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod STRB_i_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRB_i_A1_post";
    #[inline]
    pub const fn STRB_i_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod STRB_i_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRB_i_A1_pre";
    #[inline]
    pub const fn STRB_i_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_i_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_i_A1_off";
    #[inline]
    pub const fn LDRB_i_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_i_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_i_A1_post";
    #[inline]
    pub const fn LDRB_i_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_i_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_i_A1_pre";
    #[inline]
    pub const fn LDRB_i_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0101u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod STRT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRT_A1";
    #[inline]
    pub const fn STRT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRT_A1";
    #[inline]
    pub const fn LDRT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod STRBT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRBT_A1";
    #[inline]
    pub const fn STRBT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod LDRBT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRBT_A1";
    #[inline]
    pub const fn LDRBT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0100u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
