/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_pp_rr_ {
    #[inline]
    pub fn whilege_pp_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Pd: impl Into<::aarchmrs_types::BitValue<3>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01010u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 1u32
                | u32::from(eq.into()) << 0u32,
        )
    }
}
pub mod whilehs_pp_rr_ {
    #[inline]
    pub fn whilehs_pp_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Pd: impl Into<::aarchmrs_types::BitValue<3>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01011u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 1u32
                | u32::from(eq.into()) << 0u32,
        )
    }
}
pub mod whilegt_pp_rr_ {
    #[inline]
    pub fn whilegt_pp_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Pd: impl Into<::aarchmrs_types::BitValue<3>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01010u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 1u32
                | u32::from(eq.into()) << 0u32,
        )
    }
}
pub mod whilehi_pp_rr_ {
    #[inline]
    pub fn whilehi_pp_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Pd: impl Into<::aarchmrs_types::BitValue<3>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01011u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 1u32
                | u32::from(eq.into()) << 0u32,
        )
    }
}
pub mod whilelt_pp_rr_ {
    #[inline]
    pub fn whilelt_pp_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Pd: impl Into<::aarchmrs_types::BitValue<3>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01010u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 1u32
                | u32::from(eq.into()) << 0u32,
        )
    }
}
pub mod whilelo_pp_rr_ {
    #[inline]
    pub fn whilelo_pp_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Pd: impl Into<::aarchmrs_types::BitValue<3>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01011u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 1u32
                | u32::from(eq.into()) << 0u32,
        )
    }
}
pub mod whilele_pp_rr_ {
    #[inline]
    pub fn whilele_pp_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Pd: impl Into<::aarchmrs_types::BitValue<3>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01010u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 1u32
                | u32::from(eq.into()) << 0u32,
        )
    }
}
pub mod whilels_pp_rr_ {
    #[inline]
    pub fn whilels_pp_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Pd: impl Into<::aarchmrs_types::BitValue<3>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01011u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 1u32
                | u32::from(eq.into()) << 0u32,
        )
    }
}
