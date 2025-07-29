/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SXTAB16_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110100000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTAB16_A1";
    #[inline]
    pub const fn SXTAB16_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b01101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SXTB16_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110100011110000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTB16_A1";
    #[inline]
    pub const fn SXTB16_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b011010001111u32 << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SXTAB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110101000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTAB_A1";
    #[inline]
    pub const fn SXTAB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b01101010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SXTB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110101011110000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTB_A1";
    #[inline]
    pub const fn SXTB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b011010101111u32 << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SXTAH_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110101100000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTAH_A1";
    #[inline]
    pub const fn SXTAH_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b01101011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SXTH_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110101111110000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SXTH_A1";
    #[inline]
    pub const fn SXTH_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b011010111111u32 << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTAB16_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110110000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTAB16_A1";
    #[inline]
    pub const fn UXTAB16_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b01101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTB16_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110110011110000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTB16_A1";
    #[inline]
    pub const fn UXTB16_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b011011001111u32 << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTAB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110111000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTAB_A1";
    #[inline]
    pub const fn UXTAB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b01101110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110111011110000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTB_A1";
    #[inline]
    pub const fn UXTB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b011011101111u32 << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTAH_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110111100000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTAH_A1";
    #[inline]
    pub const fn UXTAH_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b01101111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UXTH_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000001111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110111111110000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UXTH_A1";
    #[inline]
    pub const fn UXTH_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        rotate: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b011011111111u32 << 16u32
                | Rd.into_inner() << 12u32
                | rotate.into_inner() << 10u32
                | 0b000111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
