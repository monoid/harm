/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SADD8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SADD8_T1";
    #[inline]
    pub const fn SADD8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QADD8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100000001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QADD8_T1";
    #[inline]
    pub const fn QADD8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SHADD8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100000001111000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHADD8_T1";
    #[inline]
    pub const fn SHADD8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UADD8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100000001111000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UADD8_T1";
    #[inline]
    pub const fn UADD8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0100u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UQADD8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100000001111000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQADD8_T1";
    #[inline]
    pub const fn UQADD8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UHADD8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100000001111000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UHADD8_T1";
    #[inline]
    pub const fn UHADD8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SADD16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SADD16_T1";
    #[inline]
    pub const fn SADD16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QADD16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100100001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QADD16_T1";
    #[inline]
    pub const fn QADD16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SHADD16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100100001111000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHADD16_T1";
    #[inline]
    pub const fn SHADD16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UADD16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100100001111000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UADD16_T1";
    #[inline]
    pub const fn UADD16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0100u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UQADD16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100100001111000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQADD16_T1";
    #[inline]
    pub const fn UQADD16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UHADD16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100100001111000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UHADD16_T1";
    #[inline]
    pub const fn UHADD16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SASX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010101000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SASX_T1";
    #[inline]
    pub const fn SASX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QASX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010101000001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QASX_T1";
    #[inline]
    pub const fn QASX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SHASX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010101000001111000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHASX_T1";
    #[inline]
    pub const fn SHASX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UASX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010101000001111000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UASX_T1";
    #[inline]
    pub const fn UASX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0100u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UQASX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010101000001111000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQASX_T1";
    #[inline]
    pub const fn UQASX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UHASX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010101000001111000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UHASX_T1";
    #[inline]
    pub const fn UHASX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SSUB8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSUB8_T1";
    #[inline]
    pub const fn SSUB8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QSUB8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110000001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QSUB8_T1";
    #[inline]
    pub const fn QSUB8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SHSUB8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110000001111000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHSUB8_T1";
    #[inline]
    pub const fn SHSUB8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod USUB8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110000001111000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USUB8_T1";
    #[inline]
    pub const fn USUB8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0100u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UQSUB8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110000001111000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQSUB8_T1";
    #[inline]
    pub const fn UQSUB8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UHSUB8_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110000001111000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UHSUB8_T1";
    #[inline]
    pub const fn UHSUB8_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SSUB16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSUB16_T1";
    #[inline]
    pub const fn SSUB16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QSUB16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110100001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QSUB16_T1";
    #[inline]
    pub const fn QSUB16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SHSUB16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110100001111000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHSUB16_T1";
    #[inline]
    pub const fn SHSUB16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod USUB16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110100001111000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USUB16_T1";
    #[inline]
    pub const fn USUB16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0100u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UQSUB16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110100001111000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQSUB16_T1";
    #[inline]
    pub const fn UQSUB16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UHSUB16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110100001111000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UHSUB16_T1";
    #[inline]
    pub const fn UHSUB16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SSAX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSAX_T1";
    #[inline]
    pub const fn SSAX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QSAX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010111000001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QSAX_T1";
    #[inline]
    pub const fn QSAX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SHSAX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010111000001111000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHSAX_T1";
    #[inline]
    pub const fn SHSAX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod USAX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010111000001111000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USAX_T1";
    #[inline]
    pub const fn USAX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0100u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UQSAX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010111000001111000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQSAX_T1";
    #[inline]
    pub const fn UQSAX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UHSAX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010111000001111000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UHSAX_T1";
    #[inline]
    pub const fn UHSAX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
