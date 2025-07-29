/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SXTAH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010000000001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTAH_T1";
    #[inline]
    pub const fn SXTAH_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110100000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SXTH_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010000011111111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTH_T2";
    #[inline]
    pub const fn SXTH_T2(
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111010000011111111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTAH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010000100001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTAH_T1";
    #[inline]
    pub const fn UXTAH_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110100001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTH_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010000111111111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTH_T2";
    #[inline]
    pub const fn UXTH_T2(
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111010000111111111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SXTAB16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010001000001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTAB16_T1";
    #[inline]
    pub const fn SXTAB16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110100010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SXTB16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010001011111111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTB16_T1";
    #[inline]
    pub const fn SXTB16_T1(
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111010001011111111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTAB16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010001100001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTAB16_T1";
    #[inline]
    pub const fn UXTAB16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110100011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTB16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010001111111111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTB16_T1";
    #[inline]
    pub const fn UXTB16_T1(
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111010001111111111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SXTAB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010010000001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTAB_T1";
    #[inline]
    pub const fn SXTAB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110100100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SXTB_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010010011111111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTB_T2";
    #[inline]
    pub const fn SXTB_T2(
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111010010011111111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTAB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010010100001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTAB_T1";
    #[inline]
    pub const fn UXTAB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110100101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTB_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010010111111111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTB_T2";
    #[inline]
    pub const fn UXTB_T2(
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111010010111111111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b10u32 << 6u32
                | rotate.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
