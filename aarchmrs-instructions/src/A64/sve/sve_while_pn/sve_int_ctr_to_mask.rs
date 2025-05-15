/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pext_pn_rr_ {
    #[inline]
    pub fn pext_pn_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm2: impl Into<::aarchmrs_types::BitValue<2>>,
        PNn: impl Into<::aarchmrs_types::BitValue<3>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b100000011100u32 << 10u32
                | u32::from(imm2.into()) << 8u32
                | u32::from(PNn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod pext_pp_rr_ {
    #[inline]
    pub fn pext_pp_rr_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        PNn: impl Into<::aarchmrs_types::BitValue<3>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1000000111010u32 << 9u32
                | u32::from(i1.into()) << 8u32
                | u32::from(PNn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
