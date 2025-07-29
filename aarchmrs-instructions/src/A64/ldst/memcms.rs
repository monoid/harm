/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CPYFP_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFP_CPY_memcms";
    #[inline]
    pub const fn CPYFP_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPWT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000000001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPWT_CPY_memcms";
    #[inline]
    pub const fn CPYFPWT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPRT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000000010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPRT_CPY_memcms";
    #[inline]
    pub const fn CPYFPRT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000000011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPT_CPY_memcms";
    #[inline]
    pub const fn CPYFPT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPWN_CPY_memcms";
    #[inline]
    pub const fn CPYFPWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPWTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPWTWN_CPY_memcms";
    #[inline]
    pub const fn CPYFPWTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPRTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPRTWN_CPY_memcms";
    #[inline]
    pub const fn CPYFPRTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000000111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPTWN_CPY_memcms";
    #[inline]
    pub const fn CPYFPTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPRN_CPY_memcms";
    #[inline]
    pub const fn CPYFPRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPWTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPWTRN_CPY_memcms";
    #[inline]
    pub const fn CPYFPWTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPRTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPRTRN_CPY_memcms";
    #[inline]
    pub const fn CPYFPRTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPTRN_CPY_memcms";
    #[inline]
    pub const fn CPYFPTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPN_CPY_memcms";
    #[inline]
    pub const fn CPYFPN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPWTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPWTN_CPY_memcms";
    #[inline]
    pub const fn CPYFPWTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPRTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPRTN_CPY_memcms";
    #[inline]
    pub const fn CPYFPRTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFPTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFPTN_CPY_memcms";
    #[inline]
    pub const fn CPYFPTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFM_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFM_CPY_memcms";
    #[inline]
    pub const fn CPYFM_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMWT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000000001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMWT_CPY_memcms";
    #[inline]
    pub const fn CPYFMWT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMRT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000000010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMRT_CPY_memcms";
    #[inline]
    pub const fn CPYFMRT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000000011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMT_CPY_memcms";
    #[inline]
    pub const fn CPYFMT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMWN_CPY_memcms";
    #[inline]
    pub const fn CPYFMWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMWTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMWTWN_CPY_memcms";
    #[inline]
    pub const fn CPYFMWTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMRTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMRTWN_CPY_memcms";
    #[inline]
    pub const fn CPYFMRTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000000111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMTWN_CPY_memcms";
    #[inline]
    pub const fn CPYFMTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMRN_CPY_memcms";
    #[inline]
    pub const fn CPYFMRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMWTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMWTRN_CPY_memcms";
    #[inline]
    pub const fn CPYFMWTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMRTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMRTRN_CPY_memcms";
    #[inline]
    pub const fn CPYFMRTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMTRN_CPY_memcms";
    #[inline]
    pub const fn CPYFMTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMN_CPY_memcms";
    #[inline]
    pub const fn CPYFMN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMWTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMWTN_CPY_memcms";
    #[inline]
    pub const fn CPYFMWTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMRTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMRTN_CPY_memcms";
    #[inline]
    pub const fn CPYFMRTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFMTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001010000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFMTN_CPY_memcms";
    #[inline]
    pub const fn CPYFMTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFE_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFE_CPY_memcms";
    #[inline]
    pub const fn CPYFE_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFEWT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000000001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFEWT_CPY_memcms";
    #[inline]
    pub const fn CPYFEWT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFERT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000000010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFERT_CPY_memcms";
    #[inline]
    pub const fn CPYFERT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFET_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000000011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFET_CPY_memcms";
    #[inline]
    pub const fn CPYFET_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFEWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFEWN_CPY_memcms";
    #[inline]
    pub const fn CPYFEWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFEWTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFEWTWN_CPY_memcms";
    #[inline]
    pub const fn CPYFEWTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFERTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFERTWN_CPY_memcms";
    #[inline]
    pub const fn CPYFERTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFETWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000000111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFETWN_CPY_memcms";
    #[inline]
    pub const fn CPYFETWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFERN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFERN_CPY_memcms";
    #[inline]
    pub const fn CPYFERN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFEWTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFEWTRN_CPY_memcms";
    #[inline]
    pub const fn CPYFEWTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFERTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFERTRN_CPY_memcms";
    #[inline]
    pub const fn CPYFERTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFETRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFETRN_CPY_memcms";
    #[inline]
    pub const fn CPYFETRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFEN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFEN_CPY_memcms";
    #[inline]
    pub const fn CPYFEN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFEWTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFEWTN_CPY_memcms";
    #[inline]
    pub const fn CPYFEWTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFERTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFERTN_CPY_memcms";
    #[inline]
    pub const fn CPYFERTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYFETN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYFETN_CPY_memcms";
    #[inline]
    pub const fn CPYFETN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETP_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETP_SET_memcms";
    #[inline]
    pub const fn SETP_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETPT_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000000001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETPT_SET_memcms";
    #[inline]
    pub const fn SETPT_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETPN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000000010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETPN_SET_memcms";
    #[inline]
    pub const fn SETPN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETPTN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000000011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETPTN_SET_memcms";
    #[inline]
    pub const fn SETPTN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETM_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETM_SET_memcms";
    #[inline]
    pub const fn SETM_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETMT_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETMT_SET_memcms";
    #[inline]
    pub const fn SETMT_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETMN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETMN_SET_memcms";
    #[inline]
    pub const fn SETMN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETMTN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000000111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETMTN_SET_memcms";
    #[inline]
    pub const fn SETMTN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETE_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETE_SET_memcms";
    #[inline]
    pub const fn SETE_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETET_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETET_SET_memcms";
    #[inline]
    pub const fn SETET_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETEN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETEN_SET_memcms";
    #[inline]
    pub const fn SETEN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETETN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011001110000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETETN_SET_memcms";
    #[inline]
    pub const fn SETETN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011001110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYP_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYP_CPY_memcms";
    #[inline]
    pub const fn CPYP_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPWT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000000001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPWT_CPY_memcms";
    #[inline]
    pub const fn CPYPWT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPRT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000000010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPRT_CPY_memcms";
    #[inline]
    pub const fn CPYPRT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000000011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPT_CPY_memcms";
    #[inline]
    pub const fn CPYPT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPWN_CPY_memcms";
    #[inline]
    pub const fn CPYPWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPWTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPWTWN_CPY_memcms";
    #[inline]
    pub const fn CPYPWTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPRTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPRTWN_CPY_memcms";
    #[inline]
    pub const fn CPYPRTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000000111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPTWN_CPY_memcms";
    #[inline]
    pub const fn CPYPTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPRN_CPY_memcms";
    #[inline]
    pub const fn CPYPRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPWTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPWTRN_CPY_memcms";
    #[inline]
    pub const fn CPYPWTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPRTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPRTRN_CPY_memcms";
    #[inline]
    pub const fn CPYPRTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPTRN_CPY_memcms";
    #[inline]
    pub const fn CPYPTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPN_CPY_memcms";
    #[inline]
    pub const fn CPYPN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPWTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPWTN_CPY_memcms";
    #[inline]
    pub const fn CPYPWTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPRTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPRTN_CPY_memcms";
    #[inline]
    pub const fn CPYPRTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYPTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYPTN_CPY_memcms";
    #[inline]
    pub const fn CPYPTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYM_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYM_CPY_memcms";
    #[inline]
    pub const fn CPYM_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMWT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000000001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMWT_CPY_memcms";
    #[inline]
    pub const fn CPYMWT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMRT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000000010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMRT_CPY_memcms";
    #[inline]
    pub const fn CPYMRT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000000011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMT_CPY_memcms";
    #[inline]
    pub const fn CPYMT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMWN_CPY_memcms";
    #[inline]
    pub const fn CPYMWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMWTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMWTWN_CPY_memcms";
    #[inline]
    pub const fn CPYMWTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMRTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMRTWN_CPY_memcms";
    #[inline]
    pub const fn CPYMRTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000000111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMTWN_CPY_memcms";
    #[inline]
    pub const fn CPYMTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMRN_CPY_memcms";
    #[inline]
    pub const fn CPYMRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMWTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMWTRN_CPY_memcms";
    #[inline]
    pub const fn CPYMWTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMRTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMRTRN_CPY_memcms";
    #[inline]
    pub const fn CPYMRTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMTRN_CPY_memcms";
    #[inline]
    pub const fn CPYMTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMN_CPY_memcms";
    #[inline]
    pub const fn CPYMN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMWTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMWTN_CPY_memcms";
    #[inline]
    pub const fn CPYMWTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMRTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMRTN_CPY_memcms";
    #[inline]
    pub const fn CPYMRTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYMTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101010000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYMTN_CPY_memcms";
    #[inline]
    pub const fn CPYMTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101010u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYE_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYE_CPY_memcms";
    #[inline]
    pub const fn CPYE_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYEWT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000000001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYEWT_CPY_memcms";
    #[inline]
    pub const fn CPYEWT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYERT_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000000010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYERT_CPY_memcms";
    #[inline]
    pub const fn CPYERT_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYET_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000000011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYET_CPY_memcms";
    #[inline]
    pub const fn CPYET_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYEWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYEWN_CPY_memcms";
    #[inline]
    pub const fn CPYEWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYEWTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYEWTWN_CPY_memcms";
    #[inline]
    pub const fn CPYEWTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYERTWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYERTWN_CPY_memcms";
    #[inline]
    pub const fn CPYERTWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYETWN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000000111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYETWN_CPY_memcms";
    #[inline]
    pub const fn CPYETWN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYERN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYERN_CPY_memcms";
    #[inline]
    pub const fn CPYERN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYEWTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYEWTRN_CPY_memcms";
    #[inline]
    pub const fn CPYEWTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYERTRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYERTRN_CPY_memcms";
    #[inline]
    pub const fn CPYERTRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYETRN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYETRN_CPY_memcms";
    #[inline]
    pub const fn CPYETRN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYEN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYEN_CPY_memcms";
    #[inline]
    pub const fn CPYEN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYEWTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYEWTN_CPY_memcms";
    #[inline]
    pub const fn CPYEWTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b110101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYERTN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYERTN_CPY_memcms";
    #[inline]
    pub const fn CPYERTN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CPYETN_CPY_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CPYETN_CPY_memcms";
    #[inline]
    pub const fn CPYETN_CPY_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101100u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGP_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGP_SET_memcms";
    #[inline]
    pub const fn SETGP_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGPT_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000000001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGPT_SET_memcms";
    #[inline]
    pub const fn SETGPT_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b000101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGPN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000000010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGPN_SET_memcms";
    #[inline]
    pub const fn SETGPN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGPTN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000000011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGPTN_SET_memcms";
    #[inline]
    pub const fn SETGPTN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b001101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGM_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGM_SET_memcms";
    #[inline]
    pub const fn SETGM_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGMT_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGMT_SET_memcms";
    #[inline]
    pub const fn SETGMT_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGMN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGMN_SET_memcms";
    #[inline]
    pub const fn SETGMN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGMTN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000000111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGMTN_SET_memcms";
    #[inline]
    pub const fn SETGMTN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGE_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGE_SET_memcms";
    #[inline]
    pub const fn SETGE_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGET_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGET_SET_memcms";
    #[inline]
    pub const fn SETGET_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b100101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGEN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGEN_SET_memcms";
    #[inline]
    pub const fn SETGEN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SETGETN_SET_memcms {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011101110000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETGETN_SET_memcms";
    #[inline]
    pub const fn SETGETN_SET_memcms(
        sz: ::aarchmrs_types::BitValue<2>,
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            sz.into_inner() << 30u32
                | 0b011101110u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b101101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
