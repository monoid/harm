/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDRD_l_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011111110000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010011110000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000001001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRD_l_A1";
    #[inline]
    pub const fn LDRD_l_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b1001111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_l_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001110010111110000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010111110000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_l_A1";
    #[inline]
    pub const fn LDRH_l_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        P: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b000u32 << 25u32
                | P.into_inner() << 24u32
                | U.into_inner() << 23u32
                | 0b1u32 << 22u32
                | W.into_inner() << 21u32
                | 0b11111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRSB_l_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001110010111110000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010111110000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSB_l_A1";
    #[inline]
    pub const fn LDRSB_l_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        P: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b000u32 << 25u32
                | P.into_inner() << 24u32
                | U.into_inner() << 23u32
                | 0b1u32 << 22u32
                | W.into_inner() << 21u32
                | 0b11111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRSH_l_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001110010111110000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010111110000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSH_l_A1";
    #[inline]
    pub const fn LDRSH_l_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        P: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b000u32 << 25u32
                | P.into_inner() << 24u32
                | U.into_inner() << 23u32
                | 0b1u32 << 22u32
                | W.into_inner() << 21u32
                | 0b11111u32 << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRD_i_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010000000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRD_i_A1_off";
    #[inline]
    pub const fn LDRD_i_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRD_i_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010000000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRD_i_A1_post";
    #[inline]
    pub const fn LDRD_i_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRD_i_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011000000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRD_i_A1_pre";
    #[inline]
    pub const fn LDRD_i_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod STRH_i_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010000000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRH_i_A1_off";
    #[inline]
    pub const fn STRH_i_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod STRH_i_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010000000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRH_i_A1_post";
    #[inline]
    pub const fn STRH_i_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod STRH_i_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011000000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRH_i_A1_pre";
    #[inline]
    pub const fn STRH_i_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod STRD_i_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010000000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRD_i_A1_off";
    #[inline]
    pub const fn STRD_i_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod STRD_i_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010000000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRD_i_A1_post";
    #[inline]
    pub const fn STRD_i_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod STRD_i_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011000000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRD_i_A1_pre";
    #[inline]
    pub const fn STRD_i_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_i_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010100000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_i_A1_off";
    #[inline]
    pub const fn LDRH_i_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_i_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010100000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_i_A1_post";
    #[inline]
    pub const fn LDRH_i_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_i_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011100000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_i_A1_pre";
    #[inline]
    pub const fn LDRH_i_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRSB_i_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010100000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSB_i_A1_off";
    #[inline]
    pub const fn LDRSB_i_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRSB_i_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010100000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSB_i_A1_post";
    #[inline]
    pub const fn LDRSB_i_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRSB_i_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011100000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSB_i_A1_pre";
    #[inline]
    pub const fn LDRSB_i_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRSH_i_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSH_i_A1_off";
    #[inline]
    pub const fn LDRSH_i_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRSH_i_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSH_i_A1_post";
    #[inline]
    pub const fn LDRSH_i_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRSH_i_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSH_i_A1_pre";
    #[inline]
    pub const fn LDRSH_i_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod STRHT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000011000000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRHT_A1";
    #[inline]
    pub const fn STRHT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRHT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000011100000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRHT_A1";
    #[inline]
    pub const fn LDRHT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRSBT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000011100000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSBT_A1";
    #[inline]
    pub const fn LDRSBT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod LDRSHT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000011100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSHT_A1";
    #[inline]
    pub const fn LDRSHT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | imm4H.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
