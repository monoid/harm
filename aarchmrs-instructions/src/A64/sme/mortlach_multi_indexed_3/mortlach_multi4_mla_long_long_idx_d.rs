/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzi_d4xi {
    #[inline]
    pub fn smlall_za_zzi_d4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i3h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<2>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011001u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b00u32 << 11u32
                | u32::from(i3h.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | u32::from(i3l.into()) << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
pub mod smlsll_za_zzi_d4xi {
    #[inline]
    pub fn smlsll_za_zzi_d4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i3h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<2>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011001u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b00u32 << 11u32
                | u32::from(i3h.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | u32::from(i3l.into()) << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
pub mod umlall_za_zzi_d4xi {
    #[inline]
    pub fn umlall_za_zzi_d4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i3h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<2>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011001u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b00u32 << 11u32
                | u32::from(i3h.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | u32::from(i3l.into()) << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
pub mod umlsll_za_zzi_d4xi {
    #[inline]
    pub fn umlsll_za_zzi_d4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i3h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<2>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011001u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b00u32 << 11u32
                | u32::from(i3h.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | u32::from(i3l.into()) << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
