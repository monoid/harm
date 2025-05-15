/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzw_2x2 {
    #[inline]
    pub fn smlall_za_zzw_2x2(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b00u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | 0b00u32 << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
pub mod usmlall_za_zzw_s2x2 {
    #[inline]
    pub fn usmlall_za_zzw_s2x2(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b00u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b00010u32 << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
pub mod smlsll_za_zzw_2x2 {
    #[inline]
    pub fn smlsll_za_zzw_2x2(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b00u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | 0b00u32 << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
pub mod umlall_za_zzw_2x2 {
    #[inline]
    pub fn umlall_za_zzw_2x2(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b00u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | 0b00u32 << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
pub mod umlsll_za_zzw_2x2 {
    #[inline]
    pub fn umlsll_za_zzw_2x2(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b00u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | 0b00u32 << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
