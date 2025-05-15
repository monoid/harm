/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CPYFP_CPY_memcms {
    #[inline]
    pub fn CPYFP_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPWT_CPY_memcms {
    #[inline]
    pub fn CPYFPWT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPRT_CPY_memcms {
    #[inline]
    pub fn CPYFPRT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPT_CPY_memcms {
    #[inline]
    pub fn CPYFPT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPWN_CPY_memcms {
    #[inline]
    pub fn CPYFPWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPWTWN_CPY_memcms {
    #[inline]
    pub fn CPYFPWTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPRTWN_CPY_memcms {
    #[inline]
    pub fn CPYFPRTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPTWN_CPY_memcms {
    #[inline]
    pub fn CPYFPTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPRN_CPY_memcms {
    #[inline]
    pub fn CPYFPRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPWTRN_CPY_memcms {
    #[inline]
    pub fn CPYFPWTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPRTRN_CPY_memcms {
    #[inline]
    pub fn CPYFPRTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPTRN_CPY_memcms {
    #[inline]
    pub fn CPYFPTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPN_CPY_memcms {
    #[inline]
    pub fn CPYFPN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPWTN_CPY_memcms {
    #[inline]
    pub fn CPYFPWTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPRTN_CPY_memcms {
    #[inline]
    pub fn CPYFPRTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFPTN_CPY_memcms {
    #[inline]
    pub fn CPYFPTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFM_CPY_memcms {
    #[inline]
    pub fn CPYFM_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMWT_CPY_memcms {
    #[inline]
    pub fn CPYFMWT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMRT_CPY_memcms {
    #[inline]
    pub fn CPYFMRT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMT_CPY_memcms {
    #[inline]
    pub fn CPYFMT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMWN_CPY_memcms {
    #[inline]
    pub fn CPYFMWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMWTWN_CPY_memcms {
    #[inline]
    pub fn CPYFMWTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMRTWN_CPY_memcms {
    #[inline]
    pub fn CPYFMRTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMTWN_CPY_memcms {
    #[inline]
    pub fn CPYFMTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMRN_CPY_memcms {
    #[inline]
    pub fn CPYFMRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMWTRN_CPY_memcms {
    #[inline]
    pub fn CPYFMWTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMRTRN_CPY_memcms {
    #[inline]
    pub fn CPYFMRTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMTRN_CPY_memcms {
    #[inline]
    pub fn CPYFMTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMN_CPY_memcms {
    #[inline]
    pub fn CPYFMN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMWTN_CPY_memcms {
    #[inline]
    pub fn CPYFMWTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMRTN_CPY_memcms {
    #[inline]
    pub fn CPYFMRTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFMTN_CPY_memcms {
    #[inline]
    pub fn CPYFMTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFE_CPY_memcms {
    #[inline]
    pub fn CPYFE_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFEWT_CPY_memcms {
    #[inline]
    pub fn CPYFEWT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFERT_CPY_memcms {
    #[inline]
    pub fn CPYFERT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFET_CPY_memcms {
    #[inline]
    pub fn CPYFET_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFEWN_CPY_memcms {
    #[inline]
    pub fn CPYFEWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFEWTWN_CPY_memcms {
    #[inline]
    pub fn CPYFEWTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFERTWN_CPY_memcms {
    #[inline]
    pub fn CPYFERTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFETWN_CPY_memcms {
    #[inline]
    pub fn CPYFETWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFERN_CPY_memcms {
    #[inline]
    pub fn CPYFERN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFEWTRN_CPY_memcms {
    #[inline]
    pub fn CPYFEWTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFERTRN_CPY_memcms {
    #[inline]
    pub fn CPYFERTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFETRN_CPY_memcms {
    #[inline]
    pub fn CPYFETRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFEN_CPY_memcms {
    #[inline]
    pub fn CPYFEN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFEWTN_CPY_memcms {
    #[inline]
    pub fn CPYFEWTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFERTN_CPY_memcms {
    #[inline]
    pub fn CPYFERTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYFETN_CPY_memcms {
    #[inline]
    pub fn CPYFETN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETP_SET_memcms {
    #[inline]
    pub fn SETP_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETPT_SET_memcms {
    #[inline]
    pub fn SETPT_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETPN_SET_memcms {
    #[inline]
    pub fn SETPN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETPTN_SET_memcms {
    #[inline]
    pub fn SETPTN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETM_SET_memcms {
    #[inline]
    pub fn SETM_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETMT_SET_memcms {
    #[inline]
    pub fn SETMT_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETMN_SET_memcms {
    #[inline]
    pub fn SETMN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETMTN_SET_memcms {
    #[inline]
    pub fn SETMTN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETE_SET_memcms {
    #[inline]
    pub fn SETE_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETET_SET_memcms {
    #[inline]
    pub fn SETET_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETEN_SET_memcms {
    #[inline]
    pub fn SETEN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETETN_SET_memcms {
    #[inline]
    pub fn SETETN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011001110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYP_CPY_memcms {
    #[inline]
    pub fn CPYP_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPWT_CPY_memcms {
    #[inline]
    pub fn CPYPWT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPRT_CPY_memcms {
    #[inline]
    pub fn CPYPRT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPT_CPY_memcms {
    #[inline]
    pub fn CPYPT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPWN_CPY_memcms {
    #[inline]
    pub fn CPYPWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPWTWN_CPY_memcms {
    #[inline]
    pub fn CPYPWTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPRTWN_CPY_memcms {
    #[inline]
    pub fn CPYPRTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPTWN_CPY_memcms {
    #[inline]
    pub fn CPYPTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPRN_CPY_memcms {
    #[inline]
    pub fn CPYPRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPWTRN_CPY_memcms {
    #[inline]
    pub fn CPYPWTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPRTRN_CPY_memcms {
    #[inline]
    pub fn CPYPRTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPTRN_CPY_memcms {
    #[inline]
    pub fn CPYPTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPN_CPY_memcms {
    #[inline]
    pub fn CPYPN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPWTN_CPY_memcms {
    #[inline]
    pub fn CPYPWTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPRTN_CPY_memcms {
    #[inline]
    pub fn CPYPRTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYPTN_CPY_memcms {
    #[inline]
    pub fn CPYPTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101000u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYM_CPY_memcms {
    #[inline]
    pub fn CPYM_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMWT_CPY_memcms {
    #[inline]
    pub fn CPYMWT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMRT_CPY_memcms {
    #[inline]
    pub fn CPYMRT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMT_CPY_memcms {
    #[inline]
    pub fn CPYMT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMWN_CPY_memcms {
    #[inline]
    pub fn CPYMWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMWTWN_CPY_memcms {
    #[inline]
    pub fn CPYMWTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMRTWN_CPY_memcms {
    #[inline]
    pub fn CPYMRTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMTWN_CPY_memcms {
    #[inline]
    pub fn CPYMTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMRN_CPY_memcms {
    #[inline]
    pub fn CPYMRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMWTRN_CPY_memcms {
    #[inline]
    pub fn CPYMWTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMRTRN_CPY_memcms {
    #[inline]
    pub fn CPYMRTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMTRN_CPY_memcms {
    #[inline]
    pub fn CPYMTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMN_CPY_memcms {
    #[inline]
    pub fn CPYMN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMWTN_CPY_memcms {
    #[inline]
    pub fn CPYMWTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMRTN_CPY_memcms {
    #[inline]
    pub fn CPYMRTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYMTN_CPY_memcms {
    #[inline]
    pub fn CPYMTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101010u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYE_CPY_memcms {
    #[inline]
    pub fn CPYE_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYEWT_CPY_memcms {
    #[inline]
    pub fn CPYEWT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYERT_CPY_memcms {
    #[inline]
    pub fn CPYERT_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYET_CPY_memcms {
    #[inline]
    pub fn CPYET_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYEWN_CPY_memcms {
    #[inline]
    pub fn CPYEWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYEWTWN_CPY_memcms {
    #[inline]
    pub fn CPYEWTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYERTWN_CPY_memcms {
    #[inline]
    pub fn CPYERTWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYETWN_CPY_memcms {
    #[inline]
    pub fn CPYETWN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYERN_CPY_memcms {
    #[inline]
    pub fn CPYERN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYEWTRN_CPY_memcms {
    #[inline]
    pub fn CPYEWTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYERTRN_CPY_memcms {
    #[inline]
    pub fn CPYERTRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYETRN_CPY_memcms {
    #[inline]
    pub fn CPYETRN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYEN_CPY_memcms {
    #[inline]
    pub fn CPYEN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYEWTN_CPY_memcms {
    #[inline]
    pub fn CPYEWTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYERTN_CPY_memcms {
    #[inline]
    pub fn CPYERTN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CPYETN_CPY_memcms {
    #[inline]
    pub fn CPYETN_CPY_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101100u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGP_SET_memcms {
    #[inline]
    pub fn SETGP_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGPT_SET_memcms {
    #[inline]
    pub fn SETGPT_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGPN_SET_memcms {
    #[inline]
    pub fn SETGPN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGPTN_SET_memcms {
    #[inline]
    pub fn SETGPTN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGM_SET_memcms {
    #[inline]
    pub fn SETGM_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGMT_SET_memcms {
    #[inline]
    pub fn SETGMT_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGMN_SET_memcms {
    #[inline]
    pub fn SETGMN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGMTN_SET_memcms {
    #[inline]
    pub fn SETGMTN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGE_SET_memcms {
    #[inline]
    pub fn SETGE_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGET_SET_memcms {
    #[inline]
    pub fn SETGET_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGEN_SET_memcms {
    #[inline]
    pub fn SETGEN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SETGETN_SET_memcms {
    #[inline]
    pub fn SETGETN_SET_memcms(
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            u32::from(sz.into()) << 30u32
                | 0b011101110u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
