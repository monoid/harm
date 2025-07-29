/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SQDMLAL_asisdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000000011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQDMLAL_asisdelem_L";
    #[inline]
    pub const fn SQDMLAL_asisdelem_L(
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0011u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQDMLSL_asisdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000000111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQDMLSL_asisdelem_L";
    #[inline]
    pub const fn SQDMLSL_asisdelem_L(
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0111u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQDMULL_asisdelem_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000001011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQDMULL_asisdelem_L";
    #[inline]
    pub const fn SQDMULL_asisdelem_L(
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011111u32 << 24u32
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
pub mod SQDMULH_asisdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQDMULH_asisdelem_R";
    #[inline]
    pub const fn SQDMULH_asisdelem_R(
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1100u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQRDMULH_asisdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQRDMULH_asisdelem_R";
    #[inline]
    pub const fn SQRDMULH_asisdelem_R(
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1101u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLA_asisdelem_RH_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLA_asisdelem_RH_H";
    #[inline]
    pub const fn FMLA_asisdelem_RH_H(
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111100u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0001u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLS_asisdelem_RH_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000000101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLS_asisdelem_RH_H";
    #[inline]
    pub const fn FMLS_asisdelem_RH_H(
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111100u32 << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0101u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMUL_asisdelem_RH_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMUL_asisdelem_RH_H";
    #[inline]
    pub const fn FMUL_asisdelem_RH_H(
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111100u32 << 22u32
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
pub mod FMLA_asisdelem_R_SD {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111100000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLA_asisdelem_R_SD";
    #[inline]
    pub const fn FMLA_asisdelem_R_SD(
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111111u32 << 23u32
                | sz.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0001u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMLS_asisdelem_R_SD {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111100000000101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMLS_asisdelem_R_SD";
    #[inline]
    pub const fn FMLS_asisdelem_R_SD(
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111111u32 << 23u32
                | sz.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b0101u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMUL_asisdelem_R_SD {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111100000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMUL_asisdelem_R_SD";
    #[inline]
    pub const fn FMUL_asisdelem_R_SD(
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111111u32 << 23u32
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
pub mod SQRDMLAH_asisdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQRDMLAH_asisdelem_R";
    #[inline]
    pub const fn SQRDMLAH_asisdelem_R(
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111111u32 << 24u32
                | size.into_inner() << 22u32
                | L.into_inner() << 21u32
                | M.into_inner() << 20u32
                | Rm.into_inner() << 16u32
                | 0b1101u32 << 12u32
                | H.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQRDMLSH_asisdelem_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQRDMLSH_asisdelem_R";
    #[inline]
    pub const fn SQRDMLSH_asisdelem_R(
        size: ::aarchmrs_types::BitValue<2>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111111u32 << 24u32
                | size.into_inner() << 22u32
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
pub mod FMULX_asisdelem_RH_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMULX_asisdelem_RH_H";
    #[inline]
    pub const fn FMULX_asisdelem_RH_H(
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111111100u32 << 22u32
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
pub mod FMULX_asisdelem_R_SD {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111100000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMULX_asisdelem_R_SD";
    #[inline]
    pub const fn FMULX_asisdelem_R_SD(
        sz: ::aarchmrs_types::BitValue<1>,
        L: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111111u32 << 23u32
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
