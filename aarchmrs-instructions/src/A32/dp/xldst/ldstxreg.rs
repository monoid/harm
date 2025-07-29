/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRH_r_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000000000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRH_r_A1_off";
    #[inline]
    pub const fn STRH_r_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRH_r_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRH_r_A1_post";
    #[inline]
    pub const fn STRH_r_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRH_r_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001000000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRH_r_A1_pre";
    #[inline]
    pub const fn STRH_r_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRD_r_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000000000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRD_r_A1_off";
    #[inline]
    pub const fn LDRD_r_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRD_r_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRD_r_A1_post";
    #[inline]
    pub const fn LDRD_r_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRD_r_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001000000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRD_r_A1_pre";
    #[inline]
    pub const fn LDRD_r_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRD_r_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000000000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRD_r_A1_off";
    #[inline]
    pub const fn STRD_r_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRD_r_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRD_r_A1_post";
    #[inline]
    pub const fn STRD_r_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRD_r_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001000000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRD_r_A1_pre";
    #[inline]
    pub const fn STRD_r_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_r_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000100000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_r_A1_off";
    #[inline]
    pub const fn LDRH_r_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_r_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000100000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_r_A1_post";
    #[inline]
    pub const fn LDRH_r_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_r_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001100000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_r_A1_pre";
    #[inline]
    pub const fn LDRH_r_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRSB_r_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000100000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSB_r_A1_off";
    #[inline]
    pub const fn LDRSB_r_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRSB_r_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000100000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSB_r_A1_post";
    #[inline]
    pub const fn LDRSB_r_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRSB_r_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001100000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSB_r_A1_pre";
    #[inline]
    pub const fn LDRSB_r_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRSH_r_A1_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSH_r_A1_off";
    #[inline]
    pub const fn LDRSH_r_A1_off(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRSH_r_A1_post {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSH_r_A1_post";
    #[inline]
    pub const fn LDRSH_r_A1_post(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRSH_r_A1_pre {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSH_r_A1_pre";
    #[inline]
    pub const fn LDRSH_r_A1_pre(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0001u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod STRHT_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000001000000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRHT_A2";
    #[inline]
    pub const fn STRHT_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRHT_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000001100000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRHT_A2";
    #[inline]
    pub const fn LDRHT_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRSBT_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000001100000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSBT_A2";
    #[inline]
    pub const fn LDRSBT_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod LDRSHT_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111011100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000001100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSHT_A2";
    #[inline]
    pub const fn LDRSHT_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0000u32 << 24u32
                | U.into_inner() << 23u32
                | 0b011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b00001111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
