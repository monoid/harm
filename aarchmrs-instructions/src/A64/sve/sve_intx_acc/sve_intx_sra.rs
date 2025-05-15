/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ssra_z_zi_ {
    #[inline]
    pub fn ssra_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b11100u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod srsra_z_zi_ {
    #[inline]
    pub fn srsra_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b11101u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod usra_z_zi_ {
    #[inline]
    pub fn usra_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b11100u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod ursra_z_zi_ {
    #[inline]
    pub fn ursra_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b11101u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
