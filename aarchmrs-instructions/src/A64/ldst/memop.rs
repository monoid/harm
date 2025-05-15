/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDADDB_32_memop {
    #[inline]
    pub fn LDADDB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRB_32_memop {
    #[inline]
    pub fn LDCLRB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORB_32_memop {
    #[inline]
    pub fn LDEORB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETB_32_memop {
    #[inline]
    pub fn LDSETB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXB_32_memop {
    #[inline]
    pub fn LDSMAXB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINB_32_memop {
    #[inline]
    pub fn LDSMINB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXB_32_memop {
    #[inline]
    pub fn LDUMAXB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINB_32_memop {
    #[inline]
    pub fn LDUMINB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPB_32_memop {
    #[inline]
    pub fn SWPB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWCLR_64_memop {
    #[inline]
    pub fn RCWCLR_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSWP_64_memop {
    #[inline]
    pub fn RCWSWP_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSET_64_memop {
    #[inline]
    pub fn RCWSET_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDLB_32_memop {
    #[inline]
    pub fn LDADDLB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRLB_32_memop {
    #[inline]
    pub fn LDCLRLB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORLB_32_memop {
    #[inline]
    pub fn LDEORLB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETLB_32_memop {
    #[inline]
    pub fn LDSETLB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXLB_32_memop {
    #[inline]
    pub fn LDSMAXLB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINLB_32_memop {
    #[inline]
    pub fn LDSMINLB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXLB_32_memop {
    #[inline]
    pub fn LDUMAXLB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINLB_32_memop {
    #[inline]
    pub fn LDUMINLB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPLB_32_memop {
    #[inline]
    pub fn SWPLB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWCLRL_64_memop {
    #[inline]
    pub fn RCWCLRL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSWPL_64_memop {
    #[inline]
    pub fn RCWSWPL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSETL_64_memop {
    #[inline]
    pub fn RCWSETL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDAB_32_memop {
    #[inline]
    pub fn LDADDAB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRAB_32_memop {
    #[inline]
    pub fn LDCLRAB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORAB_32_memop {
    #[inline]
    pub fn LDEORAB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETAB_32_memop {
    #[inline]
    pub fn LDSETAB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXAB_32_memop {
    #[inline]
    pub fn LDSMAXAB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINAB_32_memop {
    #[inline]
    pub fn LDSMINAB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXAB_32_memop {
    #[inline]
    pub fn LDUMAXAB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINAB_32_memop {
    #[inline]
    pub fn LDUMINAB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPAB_32_memop {
    #[inline]
    pub fn SWPAB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWCLRA_64_memop {
    #[inline]
    pub fn RCWCLRA_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSWPA_64_memop {
    #[inline]
    pub fn RCWSWPA_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSETA_64_memop {
    #[inline]
    pub fn RCWSETA_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPRB_32L_memop {
    #[inline]
    pub fn LDAPRB_32L_memop(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011100010111111110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDALB_32_memop {
    #[inline]
    pub fn LDADDALB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRALB_32_memop {
    #[inline]
    pub fn LDCLRALB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORALB_32_memop {
    #[inline]
    pub fn LDEORALB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETALB_32_memop {
    #[inline]
    pub fn LDSETALB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXALB_32_memop {
    #[inline]
    pub fn LDSMAXALB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINALB_32_memop {
    #[inline]
    pub fn LDSMINALB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXALB_32_memop {
    #[inline]
    pub fn LDUMAXALB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINALB_32_memop {
    #[inline]
    pub fn LDUMINALB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPALB_32_memop {
    #[inline]
    pub fn SWPALB_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWCLRAL_64_memop {
    #[inline]
    pub fn RCWCLRAL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSWPAL_64_memop {
    #[inline]
    pub fn RCWSWPAL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSETAL_64_memop {
    #[inline]
    pub fn RCWSETAL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDH_32_memop {
    #[inline]
    pub fn LDADDH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRH_32_memop {
    #[inline]
    pub fn LDCLRH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORH_32_memop {
    #[inline]
    pub fn LDEORH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETH_32_memop {
    #[inline]
    pub fn LDSETH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXH_32_memop {
    #[inline]
    pub fn LDSMAXH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINH_32_memop {
    #[inline]
    pub fn LDSMINH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXH_32_memop {
    #[inline]
    pub fn LDUMAXH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINH_32_memop {
    #[inline]
    pub fn LDUMINH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPH_32_memop {
    #[inline]
    pub fn SWPH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSCLR_64_memop {
    #[inline]
    pub fn RCWSCLR_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSSWP_64_memop {
    #[inline]
    pub fn RCWSSWP_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSSET_64_memop {
    #[inline]
    pub fn RCWSSET_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDLH_32_memop {
    #[inline]
    pub fn LDADDLH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRLH_32_memop {
    #[inline]
    pub fn LDCLRLH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORLH_32_memop {
    #[inline]
    pub fn LDEORLH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETLH_32_memop {
    #[inline]
    pub fn LDSETLH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXLH_32_memop {
    #[inline]
    pub fn LDSMAXLH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINLH_32_memop {
    #[inline]
    pub fn LDSMINLH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXLH_32_memop {
    #[inline]
    pub fn LDUMAXLH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINLH_32_memop {
    #[inline]
    pub fn LDUMINLH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPLH_32_memop {
    #[inline]
    pub fn SWPLH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSCLRL_64_memop {
    #[inline]
    pub fn RCWSCLRL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSSWPL_64_memop {
    #[inline]
    pub fn RCWSSWPL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSSETL_64_memop {
    #[inline]
    pub fn RCWSSETL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDAH_32_memop {
    #[inline]
    pub fn LDADDAH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRAH_32_memop {
    #[inline]
    pub fn LDCLRAH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORAH_32_memop {
    #[inline]
    pub fn LDEORAH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETAH_32_memop {
    #[inline]
    pub fn LDSETAH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXAH_32_memop {
    #[inline]
    pub fn LDSMAXAH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINAH_32_memop {
    #[inline]
    pub fn LDSMINAH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXAH_32_memop {
    #[inline]
    pub fn LDUMAXAH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINAH_32_memop {
    #[inline]
    pub fn LDUMINAH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPAH_32_memop {
    #[inline]
    pub fn SWPAH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSCLRA_64_memop {
    #[inline]
    pub fn RCWSCLRA_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSSWPA_64_memop {
    #[inline]
    pub fn RCWSSWPA_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSSETA_64_memop {
    #[inline]
    pub fn RCWSSETA_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPRH_32L_memop {
    #[inline]
    pub fn LDAPRH_32L_memop(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111100010111111110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDALH_32_memop {
    #[inline]
    pub fn LDADDALH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRALH_32_memop {
    #[inline]
    pub fn LDCLRALH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORALH_32_memop {
    #[inline]
    pub fn LDEORALH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETALH_32_memop {
    #[inline]
    pub fn LDSETALH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXALH_32_memop {
    #[inline]
    pub fn LDSMAXALH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINALH_32_memop {
    #[inline]
    pub fn LDSMINALH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXALH_32_memop {
    #[inline]
    pub fn LDUMAXALH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINALH_32_memop {
    #[inline]
    pub fn LDUMINALH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPALH_32_memop {
    #[inline]
    pub fn SWPALH_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSCLRAL_64_memop {
    #[inline]
    pub fn RCWSCLRAL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSSWPAL_64_memop {
    #[inline]
    pub fn RCWSSWPAL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RCWSSETAL_64_memop {
    #[inline]
    pub fn RCWSSETAL_64_memop(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(S.into()) << 30u32
                | 0b111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADD_32_memop {
    #[inline]
    pub fn LDADD_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLR_32_memop {
    #[inline]
    pub fn LDCLR_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEOR_32_memop {
    #[inline]
    pub fn LDEOR_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSET_32_memop {
    #[inline]
    pub fn LDSET_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAX_32_memop {
    #[inline]
    pub fn LDSMAX_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMIN_32_memop {
    #[inline]
    pub fn LDSMIN_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAX_32_memop {
    #[inline]
    pub fn LDUMAX_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMIN_32_memop {
    #[inline]
    pub fn LDUMIN_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWP_32_memop {
    #[inline]
    pub fn SWP_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDL_32_memop {
    #[inline]
    pub fn LDADDL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRL_32_memop {
    #[inline]
    pub fn LDCLRL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORL_32_memop {
    #[inline]
    pub fn LDEORL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETL_32_memop {
    #[inline]
    pub fn LDSETL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXL_32_memop {
    #[inline]
    pub fn LDSMAXL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINL_32_memop {
    #[inline]
    pub fn LDSMINL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXL_32_memop {
    #[inline]
    pub fn LDUMAXL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINL_32_memop {
    #[inline]
    pub fn LDUMINL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPL_32_memop {
    #[inline]
    pub fn SWPL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDA_32_memop {
    #[inline]
    pub fn LDADDA_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRA_32_memop {
    #[inline]
    pub fn LDCLRA_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORA_32_memop {
    #[inline]
    pub fn LDEORA_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETA_32_memop {
    #[inline]
    pub fn LDSETA_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXA_32_memop {
    #[inline]
    pub fn LDSMAXA_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINA_32_memop {
    #[inline]
    pub fn LDSMINA_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXA_32_memop {
    #[inline]
    pub fn LDUMAXA_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINA_32_memop {
    #[inline]
    pub fn LDUMINA_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPA_32_memop {
    #[inline]
    pub fn SWPA_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPR_32L_memop {
    #[inline]
    pub fn LDAPR_32L_memop(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011100010111111110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDAL_32_memop {
    #[inline]
    pub fn LDADDAL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRAL_32_memop {
    #[inline]
    pub fn LDCLRAL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORAL_32_memop {
    #[inline]
    pub fn LDEORAL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETAL_32_memop {
    #[inline]
    pub fn LDSETAL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXAL_32_memop {
    #[inline]
    pub fn LDSMAXAL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINAL_32_memop {
    #[inline]
    pub fn LDSMINAL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXAL_32_memop {
    #[inline]
    pub fn LDUMAXAL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINAL_32_memop {
    #[inline]
    pub fn LDUMINAL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPAL_32_memop {
    #[inline]
    pub fn SWPAL_32_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADD_64_memop {
    #[inline]
    pub fn LDADD_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLR_64_memop {
    #[inline]
    pub fn LDCLR_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEOR_64_memop {
    #[inline]
    pub fn LDEOR_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSET_64_memop {
    #[inline]
    pub fn LDSET_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAX_64_memop {
    #[inline]
    pub fn LDSMAX_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMIN_64_memop {
    #[inline]
    pub fn LDSMIN_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAX_64_memop {
    #[inline]
    pub fn LDUMAX_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMIN_64_memop {
    #[inline]
    pub fn LDUMIN_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWP_64_memop {
    #[inline]
    pub fn SWP_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST64BV0_64_memop {
    #[inline]
    pub fn ST64BV0_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST64BV_64_memop {
    #[inline]
    pub fn ST64BV_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b101100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST64B_64L_memop {
    #[inline]
    pub fn ST64B_64L_memop(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100000111111100100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD64B_64L_memop {
    #[inline]
    pub fn LD64B_64L_memop(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100000111111110100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDL_64_memop {
    #[inline]
    pub fn LDADDL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRL_64_memop {
    #[inline]
    pub fn LDCLRL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORL_64_memop {
    #[inline]
    pub fn LDEORL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETL_64_memop {
    #[inline]
    pub fn LDSETL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXL_64_memop {
    #[inline]
    pub fn LDSMAXL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINL_64_memop {
    #[inline]
    pub fn LDSMINL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXL_64_memop {
    #[inline]
    pub fn LDUMAXL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINL_64_memop {
    #[inline]
    pub fn LDUMINL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPL_64_memop {
    #[inline]
    pub fn SWPL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDA_64_memop {
    #[inline]
    pub fn LDADDA_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRA_64_memop {
    #[inline]
    pub fn LDCLRA_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORA_64_memop {
    #[inline]
    pub fn LDEORA_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETA_64_memop {
    #[inline]
    pub fn LDSETA_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXA_64_memop {
    #[inline]
    pub fn LDSMAXA_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINA_64_memop {
    #[inline]
    pub fn LDSMINA_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXA_64_memop {
    #[inline]
    pub fn LDUMAXA_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINA_64_memop {
    #[inline]
    pub fn LDUMINA_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPA_64_memop {
    #[inline]
    pub fn SWPA_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPR_64L_memop {
    #[inline]
    pub fn LDAPR_64L_memop(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100010111111110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDADDAL_64_memop {
    #[inline]
    pub fn LDADDAL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDCLRAL_64_memop {
    #[inline]
    pub fn LDCLRAL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDEORAL_64_memop {
    #[inline]
    pub fn LDEORAL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSETAL_64_memop {
    #[inline]
    pub fn LDSETAL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMAXAL_64_memop {
    #[inline]
    pub fn LDSMAXAL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDSMINAL_64_memop {
    #[inline]
    pub fn LDSMINAL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMAXAL_64_memop {
    #[inline]
    pub fn LDUMAXAL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUMINAL_64_memop {
    #[inline]
    pub fn LDUMINAL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPAL_64_memop {
    #[inline]
    pub fn SWPAL_64_memop(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFADD_16 {
    #[inline]
    pub fn LDBFADD_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMAX_16 {
    #[inline]
    pub fn LDBFMAX_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMIN_16 {
    #[inline]
    pub fn LDBFMIN_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMAXNM_16 {
    #[inline]
    pub fn LDBFMAXNM_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMINNM_16 {
    #[inline]
    pub fn LDBFMINNM_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STBFADD_16 {
    #[inline]
    pub fn STBFADD_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STBFMAX_16 {
    #[inline]
    pub fn STBFMAX_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STBFMIN_16 {
    #[inline]
    pub fn STBFMIN_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STBFMAXNM_16 {
    #[inline]
    pub fn STBFMAXNM_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STBFMINNM_16 {
    #[inline]
    pub fn STBFMINNM_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STBFADDL_16 {
    #[inline]
    pub fn STBFADDL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STBFMAXL_16 {
    #[inline]
    pub fn STBFMAXL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STBFMINL_16 {
    #[inline]
    pub fn STBFMINL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STBFMAXNML_16 {
    #[inline]
    pub fn STBFMAXNML_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STBFMINNML_16 {
    #[inline]
    pub fn STBFMINNML_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod LDBFADDL_16 {
    #[inline]
    pub fn LDBFADDL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMAXL_16 {
    #[inline]
    pub fn LDBFMAXL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMINL_16 {
    #[inline]
    pub fn LDBFMINL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMAXNML_16 {
    #[inline]
    pub fn LDBFMAXNML_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMINNML_16 {
    #[inline]
    pub fn LDBFMINNML_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFADDA_16 {
    #[inline]
    pub fn LDBFADDA_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMAXA_16 {
    #[inline]
    pub fn LDBFMAXA_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMINA_16 {
    #[inline]
    pub fn LDBFMINA_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMAXNMA_16 {
    #[inline]
    pub fn LDBFMAXNMA_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMINNMA_16 {
    #[inline]
    pub fn LDBFMINNMA_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFADDAL_16 {
    #[inline]
    pub fn LDBFADDAL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMAXAL_16 {
    #[inline]
    pub fn LDBFMAXAL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMINAL_16 {
    #[inline]
    pub fn LDBFMINAL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMAXNMAL_16 {
    #[inline]
    pub fn LDBFMAXNMAL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDBFMINNMAL_16 {
    #[inline]
    pub fn LDBFMINNMAL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFADD_16 {
    #[inline]
    pub fn LDFADD_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAX_16 {
    #[inline]
    pub fn LDFMAX_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMIN_16 {
    #[inline]
    pub fn LDFMIN_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNM_16 {
    #[inline]
    pub fn LDFMAXNM_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNM_16 {
    #[inline]
    pub fn LDFMINNM_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STFADD_16 {
    #[inline]
    pub fn STFADD_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAX_16 {
    #[inline]
    pub fn STFMAX_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMIN_16 {
    #[inline]
    pub fn STFMIN_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAXNM_16 {
    #[inline]
    pub fn STFMAXNM_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMINNM_16 {
    #[inline]
    pub fn STFMINNM_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFADDL_16 {
    #[inline]
    pub fn STFADDL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAXL_16 {
    #[inline]
    pub fn STFMAXL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMINL_16 {
    #[inline]
    pub fn STFMINL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAXNML_16 {
    #[inline]
    pub fn STFMAXNML_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMINNML_16 {
    #[inline]
    pub fn STFMINNML_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod LDFADDL_16 {
    #[inline]
    pub fn LDFADDL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXL_16 {
    #[inline]
    pub fn LDFMAXL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINL_16 {
    #[inline]
    pub fn LDFMINL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNML_16 {
    #[inline]
    pub fn LDFMAXNML_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNML_16 {
    #[inline]
    pub fn LDFMINNML_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFADDA_16 {
    #[inline]
    pub fn LDFADDA_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXA_16 {
    #[inline]
    pub fn LDFMAXA_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINA_16 {
    #[inline]
    pub fn LDFMINA_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNMA_16 {
    #[inline]
    pub fn LDFMAXNMA_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNMA_16 {
    #[inline]
    pub fn LDFMINNMA_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFADDAL_16 {
    #[inline]
    pub fn LDFADDAL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXAL_16 {
    #[inline]
    pub fn LDFMAXAL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINAL_16 {
    #[inline]
    pub fn LDFMINAL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNMAL_16 {
    #[inline]
    pub fn LDFMAXNMAL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNMAL_16 {
    #[inline]
    pub fn LDFMINNMAL_16(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFADD_32 {
    #[inline]
    pub fn LDFADD_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAX_32 {
    #[inline]
    pub fn LDFMAX_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMIN_32 {
    #[inline]
    pub fn LDFMIN_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNM_32 {
    #[inline]
    pub fn LDFMAXNM_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNM_32 {
    #[inline]
    pub fn LDFMINNM_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STFADD_32 {
    #[inline]
    pub fn STFADD_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAX_32 {
    #[inline]
    pub fn STFMAX_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMIN_32 {
    #[inline]
    pub fn STFMIN_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAXNM_32 {
    #[inline]
    pub fn STFMAXNM_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMINNM_32 {
    #[inline]
    pub fn STFMINNM_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFADDL_32 {
    #[inline]
    pub fn STFADDL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAXL_32 {
    #[inline]
    pub fn STFMAXL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMINL_32 {
    #[inline]
    pub fn STFMINL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAXNML_32 {
    #[inline]
    pub fn STFMAXNML_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMINNML_32 {
    #[inline]
    pub fn STFMINNML_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod LDFADDL_32 {
    #[inline]
    pub fn LDFADDL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXL_32 {
    #[inline]
    pub fn LDFMAXL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINL_32 {
    #[inline]
    pub fn LDFMINL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNML_32 {
    #[inline]
    pub fn LDFMAXNML_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNML_32 {
    #[inline]
    pub fn LDFMINNML_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFADDA_32 {
    #[inline]
    pub fn LDFADDA_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXA_32 {
    #[inline]
    pub fn LDFMAXA_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINA_32 {
    #[inline]
    pub fn LDFMINA_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNMA_32 {
    #[inline]
    pub fn LDFMAXNMA_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNMA_32 {
    #[inline]
    pub fn LDFMINNMA_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFADDAL_32 {
    #[inline]
    pub fn LDFADDAL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXAL_32 {
    #[inline]
    pub fn LDFMAXAL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINAL_32 {
    #[inline]
    pub fn LDFMINAL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNMAL_32 {
    #[inline]
    pub fn LDFMAXNMAL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNMAL_32 {
    #[inline]
    pub fn LDFMINNMAL_32(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFADD_64 {
    #[inline]
    pub fn LDFADD_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAX_64 {
    #[inline]
    pub fn LDFMAX_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMIN_64 {
    #[inline]
    pub fn LDFMIN_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNM_64 {
    #[inline]
    pub fn LDFMAXNM_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNM_64 {
    #[inline]
    pub fn LDFMINNM_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STFADD_64 {
    #[inline]
    pub fn STFADD_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAX_64 {
    #[inline]
    pub fn STFMAX_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMIN_64 {
    #[inline]
    pub fn STFMIN_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAXNM_64 {
    #[inline]
    pub fn STFMAXNM_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMINNM_64 {
    #[inline]
    pub fn STFMINNM_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFADDL_64 {
    #[inline]
    pub fn STFADDL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAXL_64 {
    #[inline]
    pub fn STFMAXL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMINL_64 {
    #[inline]
    pub fn STFMINL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b110100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMAXNML_64 {
    #[inline]
    pub fn STFMAXNML_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod STFMINNML_64 {
    #[inline]
    pub fn STFMINNML_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod LDFADDL_64 {
    #[inline]
    pub fn LDFADDL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXL_64 {
    #[inline]
    pub fn LDFMAXL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINL_64 {
    #[inline]
    pub fn LDFMINL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNML_64 {
    #[inline]
    pub fn LDFMAXNML_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNML_64 {
    #[inline]
    pub fn LDFMINNML_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFADDA_64 {
    #[inline]
    pub fn LDFADDA_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXA_64 {
    #[inline]
    pub fn LDFMAXA_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINA_64 {
    #[inline]
    pub fn LDFMINA_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNMA_64 {
    #[inline]
    pub fn LDFMAXNMA_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNMA_64 {
    #[inline]
    pub fn LDFMINNMA_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFADDAL_64 {
    #[inline]
    pub fn LDFADDAL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXAL_64 {
    #[inline]
    pub fn LDFMAXAL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINAL_64 {
    #[inline]
    pub fn LDFMINAL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b010100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMAXNMAL_64 {
    #[inline]
    pub fn LDFMAXNMAL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDFMINNMAL_64 {
    #[inline]
    pub fn LDFMINNMAL_64(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b011100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
