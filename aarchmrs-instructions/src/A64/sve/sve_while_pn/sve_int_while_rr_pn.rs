/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_pn_rr_ {
    #[inline]
    pub fn whilege_pn_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        vl: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        PNd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(vl.into()) << 13u32
                | 0b00u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(eq.into()) << 3u32
                | u32::from(PNd.into()) << 0u32,
        )
    }
}
pub mod whilehs_pn_rr_ {
    #[inline]
    pub fn whilehs_pn_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        vl: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        PNd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(vl.into()) << 13u32
                | 0b01u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(eq.into()) << 3u32
                | u32::from(PNd.into()) << 0u32,
        )
    }
}
pub mod whilegt_pn_rr_ {
    #[inline]
    pub fn whilegt_pn_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        vl: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        PNd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(vl.into()) << 13u32
                | 0b00u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(eq.into()) << 3u32
                | u32::from(PNd.into()) << 0u32,
        )
    }
}
pub mod whilehi_pn_rr_ {
    #[inline]
    pub fn whilehi_pn_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        vl: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        PNd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(vl.into()) << 13u32
                | 0b01u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(eq.into()) << 3u32
                | u32::from(PNd.into()) << 0u32,
        )
    }
}
pub mod whilelt_pn_rr_ {
    #[inline]
    pub fn whilelt_pn_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        vl: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        PNd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(vl.into()) << 13u32
                | 0b00u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(eq.into()) << 3u32
                | u32::from(PNd.into()) << 0u32,
        )
    }
}
pub mod whilelo_pn_rr_ {
    #[inline]
    pub fn whilelo_pn_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        vl: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        PNd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(vl.into()) << 13u32
                | 0b01u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(eq.into()) << 3u32
                | u32::from(PNd.into()) << 0u32,
        )
    }
}
pub mod whilele_pn_rr_ {
    #[inline]
    pub fn whilele_pn_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        vl: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        PNd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(vl.into()) << 13u32
                | 0b00u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(eq.into()) << 3u32
                | u32::from(PNd.into()) << 0u32,
        )
    }
}
pub mod whilels_pn_rr_ {
    #[inline]
    pub fn whilels_pn_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        vl: impl Into<::aarchmrs_types::BitValue<1>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        eq: impl Into<::aarchmrs_types::BitValue<1>>,
        PNd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(vl.into()) << 13u32
                | 0b01u32 << 11u32
                | u32::from(lt.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(eq.into()) << 3u32
                | u32::from(PNd.into()) << 0u32,
        )
    }
}
