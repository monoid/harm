/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod saddlbt_z_zz_ {
    #[inline]
    pub fn saddlbt_z_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ssublbt_z_zz_ {
    #[inline]
    pub fn ssublbt_z_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b100010u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ssubltb_z_zz_ {
    #[inline]
    pub fn ssubltb_z_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b100011u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
