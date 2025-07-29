/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SMLAL_asimdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLAL_asimdelem_L";
    #[inline]
    pub const fn SMLAL_asimdelem_L(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b10u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQDMLAL_asimdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQDMLAL_asimdelem_L";
    #[inline]
    pub const fn SQDMLAL_asimdelem_L(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b11u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SMLSL_asimdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLSL_asimdelem_L";
    #[inline]
    pub const fn SMLSL_asimdelem_L(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b10u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQDMLSL_asimdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQDMLSL_asimdelem_L";
    #[inline]
    pub const fn SQDMLSL_asimdelem_L(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b11u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MUL_asimdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MUL_asimdelem_R";
    #[inline]
    pub const fn MUL_asimdelem_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SMULL_asimdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULL_asimdelem_L";
    #[inline]
    pub const fn SMULL_asimdelem_L(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQDMULL_asimdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQDMULL_asimdelem_L";
    #[inline]
    pub const fn SQDMULL_asimdelem_L(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1011u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQDMULH_asimdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQDMULH_asimdelem_R";
    #[inline]
    pub const fn SQDMULH_asimdelem_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | op.into_inner() << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQRDMULH_asimdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQRDMULH_asimdelem_R";
    #[inline]
    pub const fn SQRDMULH_asimdelem_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | op.into_inner() << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SDOT_asimdelem_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SDOT_asimdelem_D";
    #[inline]
    pub const fn SDOT_asimdelem_D(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1110u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FDOT_asimdelem_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FDOT_asimdelem_D";
    #[inline]
    pub const fn FDOT_asimdelem_D(
        Q: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLA_asimdelem_RH_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111110000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLA_asimdelem_RH_H";
    #[inline]
    pub const fn FMLA_asimdelem_RH_H(
        Q: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b01u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLS_asimdelem_RH_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111110000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLS_asimdelem_RH_H";
    #[inline]
    pub const fn FMLS_asimdelem_RH_H(
        Q: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b01u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMUL_asimdelem_RH_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMUL_asimdelem_RH_H";
    #[inline]
    pub const fn FMUL_asimdelem_RH_H(
        Q: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SUDOT_asimdelem_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111010000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUDOT_asimdelem_D";
    #[inline]
    pub const fn SUDOT_asimdelem_D(
        Q: ::aarchmrs_types::BitValue<1>,
        US: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | US.into_inner() << 23u32
                | 0b0u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FDOT_asimdelem_G {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FDOT_asimdelem_G";
    #[inline]
    pub const fn FDOT_asimdelem_G(
        Q: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111101u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BFDOT_asimdelem_E {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111010000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BFDOT_asimdelem_E";
    #[inline]
    pub const fn BFDOT_asimdelem_E(
        Q: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111101u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLA_asimdelem_R_SD {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111100000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLA_asimdelem_R_SD";
    #[inline]
    pub const fn FMLA_asimdelem_R_SD(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011111u32 << 23u32
                | sz.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b01u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLS_asimdelem_R_SD {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111100000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLS_asimdelem_R_SD";
    #[inline]
    pub const fn FMLS_asimdelem_R_SD(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011111u32 << 23u32
                | sz.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b01u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMUL_asimdelem_R_SD {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111100000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMUL_asimdelem_R_SD";
    #[inline]
    pub const fn FMUL_asimdelem_R_SD(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011111u32 << 23u32
                | sz.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLAL_asimdelem_LH {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLAL_asimdelem_LH";
    #[inline]
    pub const fn FMLAL_asimdelem_LH(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        S: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011111u32 << 23u32
                | sz.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | S.into_inner() << 14u32
                | 0b00u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLSL_asimdelem_LH {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLSL_asimdelem_LH";
    #[inline]
    pub const fn FMLSL_asimdelem_LH(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        S: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011111u32 << 23u32
                | sz.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | S.into_inner() << 14u32
                | 0b00u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod USDOT_asimdelem_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111010000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USDOT_asimdelem_D";
    #[inline]
    pub const fn USDOT_asimdelem_D(
        Q: ::aarchmrs_types::BitValue<1>,
        US: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001111u32 << 24u32
                | US.into_inner() << 23u32
                | 0b0u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BFMLAL_asimdelem_F {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111110000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BFMLAL_asimdelem_F";
    #[inline]
    pub const fn BFMLAL_asimdelem_F(
        Q: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111111u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MLA_asimdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MLA_asimdelem_R";
    #[inline]
    pub const fn MLA_asimdelem_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b00u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UMLAL_asimdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMLAL_asimdelem_L";
    #[inline]
    pub const fn UMLAL_asimdelem_L(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b10u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MLS_asimdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MLS_asimdelem_R";
    #[inline]
    pub const fn MLS_asimdelem_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b00u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UMLSL_asimdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMLSL_asimdelem_L";
    #[inline]
    pub const fn UMLSL_asimdelem_L(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | o2.into_inner() << 14u32
                | 0b10u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UMULL_asimdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMULL_asimdelem_L";
    #[inline]
    pub const fn UMULL_asimdelem_L(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQRDMLAH_asimdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQRDMLAH_asimdelem_R";
    #[inline]
    pub const fn SQRDMLAH_asimdelem_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        S: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | S.into_inner() << 13u32
                | 0b1u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UDOT_asimdelem_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UDOT_asimdelem_D";
    #[inline]
    pub const fn UDOT_asimdelem_D(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1110u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQRDMLSH_asimdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQRDMLSH_asimdelem_R";
    #[inline]
    pub const fn SQRDMLSH_asimdelem_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        S: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | S.into_inner() << 13u32
                | 0b1u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMULX_asimdelem_RH_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMULX_asimdelem_RH_H";
    #[inline]
    pub const fn FMULX_asimdelem_RH_H(
        Q: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111100u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMLA_advsimd_elt {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMLA_advsimd_elt";
    #[inline]
    pub const fn FCMLA_advsimd_elt(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        rot: ::aarchmrs_types::BitValue<2>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | rot.into_inner() << 13u32
                | 0b1u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMULX_asimdelem_R_SD {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111100000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMULX_asimdelem_R_SD";
    #[inline]
    pub const fn FMULX_asimdelem_R_SD(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011111u32 << 23u32
                | sz.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLAL2_asimdelem_LH {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111100000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLAL2_asimdelem_LH";
    #[inline]
    pub const fn FMLAL2_asimdelem_LH(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        S: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011111u32 << 23u32
                | sz.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | S.into_inner() << 14u32
                | 0b00u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLSL2_asimdelem_LH {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111100000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLSL2_asimdelem_LH";
    #[inline]
    pub const fn FMLSL2_asimdelem_LH(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        S: ::aarchmrs_types::BitValue<1>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011111u32 << 23u32
                | sz.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | S.into_inner() << 14u32
                | 0b00u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLALB_asimdelem_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLALB_asimdelem_H";
    #[inline]
    pub const fn FMLALB_asimdelem_H(
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0000111111u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLALLBB_asimdelem_J {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLALLBB_asimdelem_J";
    #[inline]
    pub const fn FMLALLBB_asimdelem_J(
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010111100u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLALLBT_asimdelem_J {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLALLBT_asimdelem_J";
    #[inline]
    pub const fn FMLALLBT_asimdelem_J(
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010111101u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLALT_asimdelem_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLALT_asimdelem_H";
    #[inline]
    pub const fn FMLALT_asimdelem_H(
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100111111u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLALLTB_asimdelem_J {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101111000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLALLTB_asimdelem_J";
    #[inline]
    pub const fn FMLALLTB_asimdelem_J(
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110111100u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLALLTT_asimdelem_J {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101111010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLALLTT_asimdelem_J";
    #[inline]
    pub const fn FMLALLTT_asimdelem_J(
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110111101u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
