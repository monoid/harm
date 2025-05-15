/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzi_s {
    #[inline]
    pub fn smlall_za_zzi_s(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        i4h: impl Into<::aarchmrs_types::BitValue<1>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i4l: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010000u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(i4h.into()) << 15u32
                | u32::from(Rv.into()) << 13u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | 0b0u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod usmlall_za_zzi_s {
    #[inline]
    pub fn usmlall_za_zzi_s(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        i4h: impl Into<::aarchmrs_types::BitValue<1>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i4l: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010000u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(i4h.into()) << 15u32
                | u32::from(Rv.into()) << 13u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(U.into()) << 4u32
                | 0b01u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod smlsll_za_zzi_s {
    #[inline]
    pub fn smlsll_za_zzi_s(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        i4h: impl Into<::aarchmrs_types::BitValue<1>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i4l: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010000u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(i4h.into()) << 15u32
                | u32::from(Rv.into()) << 13u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | 0b0u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod umlall_za_zzi_s {
    #[inline]
    pub fn umlall_za_zzi_s(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        i4h: impl Into<::aarchmrs_types::BitValue<1>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i4l: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010000u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(i4h.into()) << 15u32
                | u32::from(Rv.into()) << 13u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | 0b0u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod sumlall_za_zzi_s {
    #[inline]
    pub fn sumlall_za_zzi_s(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        i4h: impl Into<::aarchmrs_types::BitValue<1>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i4l: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010000u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(i4h.into()) << 15u32
                | u32::from(Rv.into()) << 13u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(U.into()) << 4u32
                | 0b01u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod umlsll_za_zzi_s {
    #[inline]
    pub fn umlsll_za_zzi_s(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        i4h: impl Into<::aarchmrs_types::BitValue<1>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i4l: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010000u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(i4h.into()) << 15u32
                | u32::from(Rv.into()) << 13u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | 0b0u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
