/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDTADD_32_memop_unpriv {
    #[inline]
    pub fn LDTADD_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTCLR_32_memop_unpriv {
    #[inline]
    pub fn LDTCLR_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTSET_32_memop_unpriv {
    #[inline]
    pub fn LDTSET_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPT_32_memop_unpriv {
    #[inline]
    pub fn SWPT_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTADDL_32_memop_unpriv {
    #[inline]
    pub fn LDTADDL_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTCLRL_32_memop_unpriv {
    #[inline]
    pub fn LDTCLRL_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTSETL_32_memop_unpriv {
    #[inline]
    pub fn LDTSETL_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPTL_32_memop_unpriv {
    #[inline]
    pub fn SWPTL_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTADDA_32_memop_unpriv {
    #[inline]
    pub fn LDTADDA_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTCLRA_32_memop_unpriv {
    #[inline]
    pub fn LDTCLRA_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTSETA_32_memop_unpriv {
    #[inline]
    pub fn LDTSETA_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPTA_32_memop_unpriv {
    #[inline]
    pub fn SWPTA_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTADDAL_32_memop_unpriv {
    #[inline]
    pub fn LDTADDAL_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTCLRAL_32_memop_unpriv {
    #[inline]
    pub fn LDTCLRAL_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTSETAL_32_memop_unpriv {
    #[inline]
    pub fn LDTSETAL_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPTAL_32_memop_unpriv {
    #[inline]
    pub fn SWPTAL_32_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTADD_64_memop_unpriv {
    #[inline]
    pub fn LDTADD_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTCLR_64_memop_unpriv {
    #[inline]
    pub fn LDTCLR_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTSET_64_memop_unpriv {
    #[inline]
    pub fn LDTSET_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPT_64_memop_unpriv {
    #[inline]
    pub fn SWPT_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTADDL_64_memop_unpriv {
    #[inline]
    pub fn LDTADDL_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTCLRL_64_memop_unpriv {
    #[inline]
    pub fn LDTCLRL_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTSETL_64_memop_unpriv {
    #[inline]
    pub fn LDTSETL_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPTL_64_memop_unpriv {
    #[inline]
    pub fn SWPTL_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001011u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTADDA_64_memop_unpriv {
    #[inline]
    pub fn LDTADDA_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTCLRA_64_memop_unpriv {
    #[inline]
    pub fn LDTCLRA_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTSETA_64_memop_unpriv {
    #[inline]
    pub fn LDTSETA_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPTA_64_memop_unpriv {
    #[inline]
    pub fn SWPTA_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001101u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTADDAL_64_memop_unpriv {
    #[inline]
    pub fn LDTADDAL_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTCLRAL_64_memop_unpriv {
    #[inline]
    pub fn LDTCLRAL_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTSETAL_64_memop_unpriv {
    #[inline]
    pub fn LDTSETAL_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod SWPTAL_64_memop_unpriv {
    #[inline]
    pub fn SWPTAL_64_memop_unpriv(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001111u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
