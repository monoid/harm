/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod f1cvt_mz2_z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001001101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "f1cvt_mz2_z8_";
    #[inline]
    pub const fn f1cvt_mz2_z8_(
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100110111000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod bf1cvt_mz2_z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011001101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bf1cvt_mz2_z8_";
    #[inline]
    pub const fn bf1cvt_mz2_z8_(
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000101100110111000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod f2cvt_mz2_z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101001101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "f2cvt_mz2_z8_";
    #[inline]
    pub const fn f2cvt_mz2_z8_(
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110100110111000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod bf2cvt_mz2_z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111001101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bf2cvt_mz2_z8_";
    #[inline]
    pub const fn bf2cvt_mz2_z8_(
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000111100110111000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod f1cvtl_mz2_z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001001101110000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "f1cvtl_mz2_z8_";
    #[inline]
    pub const fn f1cvtl_mz2_z8_(
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100110111000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod bf1cvtl_mz2_z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011001101110000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bf1cvtl_mz2_z8_";
    #[inline]
    pub const fn bf1cvtl_mz2_z8_(
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000101100110111000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod f2cvtl_mz2_z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101001101110000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "f2cvtl_mz2_z8_";
    #[inline]
    pub const fn f2cvtl_mz2_z8_(
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110100110111000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod bf2cvtl_mz2_z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111001101110000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bf2cvtl_mz2_z8_";
    #[inline]
    pub const fn bf2cvtl_mz2_z8_(
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000111100110111000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
