/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmphs_p_p_zi_ {
    #[inline]
    pub fn cmphs_p_p_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ne: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm7.into()) << 14u32
                | u32::from(lt.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(ne.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod cmphi_p_p_zi_ {
    #[inline]
    pub fn cmphi_p_p_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ne: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm7.into()) << 14u32
                | u32::from(lt.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(ne.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod cmplo_p_p_zi_ {
    #[inline]
    pub fn cmplo_p_p_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ne: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm7.into()) << 14u32
                | u32::from(lt.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(ne.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod cmpls_p_p_zi_ {
    #[inline]
    pub fn cmpls_p_p_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ne: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm7.into()) << 14u32
                | u32::from(lt.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(ne.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
