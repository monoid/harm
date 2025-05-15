/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sri_z_zzi_ {
    #[inline]
    pub fn sri_z_zzi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sli_z_zzi_ {
    #[inline]
    pub fn sli_z_zzi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
