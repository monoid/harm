/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmpge_p_p_zi_ {
    #[inline]
    pub fn cmpge_p_p_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ne: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(lt.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(ne.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod cmpeq_p_p_zi_ {
    #[inline]
    pub fn cmpeq_p_p_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ne: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(ne.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod cmplt_p_p_zi_ {
    #[inline]
    pub fn cmplt_p_p_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ne: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(lt.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(ne.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod cmpgt_p_p_zi_ {
    #[inline]
    pub fn cmpgt_p_p_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ne: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(lt.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(ne.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod cmpne_p_p_zi_ {
    #[inline]
    pub fn cmpne_p_p_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ne: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(ne.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod cmple_p_p_zi_ {
    #[inline]
    pub fn cmple_p_p_zi_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        lt: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        ne: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(lt.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(ne.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
