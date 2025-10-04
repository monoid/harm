/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SETGOP_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOP_memset_go";
    #[inline]
    pub const fn SETGOP_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOPT_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111110001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOPT_memset_go";
    #[inline]
    pub const fn SETGOPT_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111000100u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOPN_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111110010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOPN_memset_go";
    #[inline]
    pub const fn SETGOPN_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOPTN_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111110011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOPTN_memset_go";
    #[inline]
    pub const fn SETGOPTN_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111001100u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOM_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111110100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOM_memset_go";
    #[inline]
    pub const fn SETGOM_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111010000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOMT_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111110101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOMT_memset_go";
    #[inline]
    pub const fn SETGOMT_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111010100u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOMN_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111110110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOMN_memset_go";
    #[inline]
    pub const fn SETGOMN_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111011000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOMTN_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111110111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOMTN_memset_go";
    #[inline]
    pub const fn SETGOMTN_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111011100u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOE_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOE_memset_go";
    #[inline]
    pub const fn SETGOE_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111100000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOET_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111111001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOET_memset_go";
    #[inline]
    pub const fn SETGOET_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111100100u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOEN_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111111010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOEN_memset_go";
    #[inline]
    pub const fn SETGOEN_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111101000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGOETN_memset_go {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110111111011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGOETN_memset_go";
    #[inline]
    pub const fn SETGOETN_memset_go(
        sz: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b01110111011111101100u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
