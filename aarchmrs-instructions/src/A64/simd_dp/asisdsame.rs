/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SQADD_asisdsame_only {
    #[inline]
    pub fn SQADD_asisdsame_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQSUB_asisdsame_only {
    #[inline]
    pub fn SQSUB_asisdsame_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMGT_asisdsame_only {
    #[inline]
    pub fn CMGT_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0011u32 << 12u32
                | u32::from(eq.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMGE_asisdsame_only {
    #[inline]
    pub fn CMGE_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0011u32 << 12u32
                | u32::from(eq.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SSHL_asisdsame_only {
    #[inline]
    pub fn SSHL_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        R: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(R.into()) << 12u32
                | u32::from(S.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQSHL_asisdsame_only {
    #[inline]
    pub fn SQSHL_asisdsame_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        R: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(R.into()) << 12u32
                | u32::from(S.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SRSHL_asisdsame_only {
    #[inline]
    pub fn SRSHL_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        R: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(R.into()) << 12u32
                | u32::from(S.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQRSHL_asisdsame_only {
    #[inline]
    pub fn SQRSHL_asisdsame_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        R: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(R.into()) << 12u32
                | u32::from(S.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADD_asisdsame_only {
    #[inline]
    pub fn ADD_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMTST_asisdsame_only {
    #[inline]
    pub fn CMTST_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQDMULH_asisdsame_only {
    #[inline]
    pub fn SQDMULH_asisdsame_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b101101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMULX_asisdsame_only {
    #[inline]
    pub fn FMULX_asisdsame_only(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111100u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b110111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMEQ_asisdsame_only {
    #[inline]
    pub fn FCMEQ_asisdsame_only(
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(E.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(ac.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FRECPS_asisdsame_only {
    #[inline]
    pub fn FRECPS_asisdsame_only(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111100u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FRSQRTS_asisdsame_only {
    #[inline]
    pub fn FRSQRTS_asisdsame_only(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111101u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UQADD_asisdsame_only {
    #[inline]
    pub fn UQADD_asisdsame_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UQSUB_asisdsame_only {
    #[inline]
    pub fn UQSUB_asisdsame_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMHI_asisdsame_only {
    #[inline]
    pub fn CMHI_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0011u32 << 12u32
                | u32::from(eq.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMHS_asisdsame_only {
    #[inline]
    pub fn CMHS_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0011u32 << 12u32
                | u32::from(eq.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod USHL_asisdsame_only {
    #[inline]
    pub fn USHL_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        R: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(R.into()) << 12u32
                | u32::from(S.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UQSHL_asisdsame_only {
    #[inline]
    pub fn UQSHL_asisdsame_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        R: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(R.into()) << 12u32
                | u32::from(S.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod URSHL_asisdsame_only {
    #[inline]
    pub fn URSHL_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        R: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(R.into()) << 12u32
                | u32::from(S.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UQRSHL_asisdsame_only {
    #[inline]
    pub fn UQRSHL_asisdsame_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        R: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(R.into()) << 12u32
                | u32::from(S.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUB_asisdsame_only {
    #[inline]
    pub fn SUB_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMEQ_asisdsame_only {
    #[inline]
    pub fn CMEQ_asisdsame_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQRDMULH_asisdsame_only {
    #[inline]
    pub fn SQRDMULH_asisdsame_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b101101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMGE_asisdsame_only {
    #[inline]
    pub fn FCMGE_asisdsame_only(
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(E.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(ac.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FACGE_asisdsame_only {
    #[inline]
    pub fn FACGE_asisdsame_only(
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(E.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(ac.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FABD_asisdsame_only {
    #[inline]
    pub fn FABD_asisdsame_only(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111101u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b110101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMGT_asisdsame_only {
    #[inline]
    pub fn FCMGT_asisdsame_only(
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(E.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(ac.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FACGT_asisdsame_only {
    #[inline]
    pub fn FACGT_asisdsame_only(
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(E.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(ac.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
