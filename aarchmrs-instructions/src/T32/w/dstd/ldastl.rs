/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STLB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110000000000111110001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLB_T1";
    #[inline]
    pub const fn STLB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111110001111u32 << 0u32,
        )
    }
}
pub mod STLH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110000000000111110011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLH_T1";
    #[inline]
    pub const fn STLH_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111110011111u32 << 0u32,
        )
    }
}
pub mod STL_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110000000000111110101111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STL_T1";
    #[inline]
    pub const fn STL_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111110101111u32 << 0u32,
        )
    }
}
pub mod STLEXB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110000000000111111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLEXB_T1";
    #[inline]
    pub const fn STLEXB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b11111100u32 << 4u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod STLEXH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110000000000111111010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLEXH_T1";
    #[inline]
    pub const fn STLEXH_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b11111101u32 << 4u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod STLEX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110000000000111111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLEX_T1";
    #[inline]
    pub const fn STLEX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b11111110u32 << 4u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod STLEXD_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110000000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLEXD_T1";
    #[inline]
    pub const fn STLEXD_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | Rt2.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod LDAB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100000000111110001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAB_T1";
    #[inline]
    pub const fn LDAB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111110001111u32 << 0u32,
        )
    }
}
pub mod LDAH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100000000111110011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAH_T1";
    #[inline]
    pub const fn LDAH_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111110011111u32 << 0u32,
        )
    }
}
pub mod LDA_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100000000111110101111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDA_T1";
    #[inline]
    pub const fn LDA_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111110101111u32 << 0u32,
        )
    }
}
pub mod LDAEXB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100000000111111001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAEXB_T1";
    #[inline]
    pub const fn LDAEXB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111111001111u32 << 0u32,
        )
    }
}
pub mod LDAEXH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100000000111111011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAEXH_T1";
    #[inline]
    pub const fn LDAEXH_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111111011111u32 << 0u32,
        )
    }
}
pub mod LDAEX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100000000111111101111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAEX_T1";
    #[inline]
    pub const fn LDAEX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111111101111u32 << 0u32,
        )
    }
}
pub mod LDAEXD_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100000000000011111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAEXD_T1";
    #[inline]
    pub const fn LDAEXD_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | Rt2.into_inner() << 8u32
                | 0b11111111u32 << 0u32,
        )
    }
}
