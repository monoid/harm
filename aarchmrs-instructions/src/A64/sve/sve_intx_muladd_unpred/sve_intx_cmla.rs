/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmla_z_zzz_ {
    #[inline]
    pub fn cmla_z_zzz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        rot: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(rot.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod sqrdcmlah_z_zzz_ {
    #[inline]
    pub fn sqrdcmlah_z_zzz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        rot: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b0011u32 << 12u32
                | u32::from(rot.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
