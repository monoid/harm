/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sunpklo_z_z_ {
    #[inline]
    pub fn sunpklo_z_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1100u32 << 18u32
                | u32::from(U.into()) << 17u32
                | u32::from(H.into()) << 16u32
                | 0b001110u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sunpkhi_z_z_ {
    #[inline]
    pub fn sunpkhi_z_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1100u32 << 18u32
                | u32::from(U.into()) << 17u32
                | u32::from(H.into()) << 16u32
                | 0b001110u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uunpklo_z_z_ {
    #[inline]
    pub fn uunpklo_z_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1100u32 << 18u32
                | u32::from(U.into()) << 17u32
                | u32::from(H.into()) << 16u32
                | 0b001110u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uunpkhi_z_z_ {
    #[inline]
    pub fn uunpkhi_z_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1100u32 << 18u32
                | u32::from(U.into()) << 17u32
                | u32::from(H.into()) << 16u32
                | 0b001110u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
