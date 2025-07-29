/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zero_za1_ri_2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001111111111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000011000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zero_za1_ri_2";
    #[inline]
    pub const fn zero_za1_ri_2(
        Rv: ::aarchmrs_types::BitValue<2>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011000u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0000000000u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod zero_za1_ri_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001111111111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000011100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zero_za1_ri_4";
    #[inline]
    pub const fn zero_za1_ri_4(
        Rv: ::aarchmrs_types::BitValue<2>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011100u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0000000000u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod zero_za2_ri_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001111111111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000011001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zero_za2_ri_1";
    #[inline]
    pub const fn zero_za2_ri_1(
        Rv: ::aarchmrs_types::BitValue<2>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011001u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0000000000u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod zero_za2_ri_2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001111111111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000011010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zero_za2_ri_2";
    #[inline]
    pub const fn zero_za2_ri_2(
        Rv: ::aarchmrs_types::BitValue<2>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00000000000u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod zero_za2_ri_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001111111111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000011011000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zero_za2_ri_4";
    #[inline]
    pub const fn zero_za2_ri_4(
        Rv: ::aarchmrs_types::BitValue<2>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011011u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00000000000u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod zero_za4_ri_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001111111111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000011101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zero_za4_ri_1";
    #[inline]
    pub const fn zero_za4_ri_1(
        Rv: ::aarchmrs_types::BitValue<2>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011101u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00000000000u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod zero_za4_ri_2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001111111111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zero_za4_ri_2";
    #[inline]
    pub const fn zero_za4_ri_2(
        Rv: ::aarchmrs_types::BitValue<2>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011110u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000000000000u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod zero_za4_ri_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001111111111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000011111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zero_za4_ri_4";
    #[inline]
    pub const fn zero_za4_ri_4(
        Rv: ::aarchmrs_types::BitValue<2>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011111u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000000000000u32 << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
