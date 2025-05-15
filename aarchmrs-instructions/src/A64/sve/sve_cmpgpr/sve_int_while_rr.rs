/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_p_p_rr_ {
    #[inline]
    pub fn whilege_p_p_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        sf: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(sf.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(eq.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod whilehs_p_p_rr_ {
    #[inline]
    pub fn whilehs_p_p_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        sf: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(sf.into()) << 12u32
                | 0b1u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(eq.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod whilegt_p_p_rr_ {
    #[inline]
    pub fn whilegt_p_p_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        sf: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(sf.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(eq.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod whilehi_p_p_rr_ {
    #[inline]
    pub fn whilehi_p_p_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        sf: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(sf.into()) << 12u32
                | 0b1u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(eq.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod whilelt_p_p_rr_ {
    #[inline]
    pub fn whilelt_p_p_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        sf: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(sf.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(eq.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod whilelo_p_p_rr_ {
    #[inline]
    pub fn whilelo_p_p_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        sf: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(sf.into()) << 12u32
                | 0b1u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(eq.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod whilele_p_p_rr_ {
    #[inline]
    pub fn whilele_p_p_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        sf: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(sf.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(eq.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod whilels_p_p_rr_ {
    #[inline]
    pub fn whilels_p_p_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        sf: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(sf.into()) << 12u32
                | 0b1u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(eq.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
