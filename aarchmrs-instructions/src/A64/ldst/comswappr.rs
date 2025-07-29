/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CASP_CP32_comswappr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000001000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CASP_CP32_comswappr";
    #[inline]
    pub const fn CASP_CP32_comswappr(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001000001u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CASPL_CP32_comswappr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000001000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CASPL_CP32_comswappr";
    #[inline]
    pub const fn CASPL_CP32_comswappr(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001000001u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CASPA_CP32_comswappr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000011000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CASPA_CP32_comswappr";
    #[inline]
    pub const fn CASPA_CP32_comswappr(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001000011u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CASPAL_CP32_comswappr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000011000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CASPAL_CP32_comswappr";
    #[inline]
    pub const fn CASPAL_CP32_comswappr(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001000011u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CASP_CP64_comswappr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001000001000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CASP_CP64_comswappr";
    #[inline]
    pub const fn CASP_CP64_comswappr(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001000001u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CASPL_CP64_comswappr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001000001000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CASPL_CP64_comswappr";
    #[inline]
    pub const fn CASPL_CP64_comswappr(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001000001u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CASPA_CP64_comswappr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001000011000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CASPA_CP64_comswappr";
    #[inline]
    pub const fn CASPA_CP64_comswappr(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001000011u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CASPAL_CP64_comswappr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001000011000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CASPAL_CP64_comswappr";
    #[inline]
    pub const fn CASPAL_CP64_comswappr(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001000011u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
