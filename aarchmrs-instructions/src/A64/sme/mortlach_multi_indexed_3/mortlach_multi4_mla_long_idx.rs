/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlal_za_zzi_4xi {
    #[inline]
    pub fn smlal_za_zzi_4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i3h.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | u32::from(i3l.into()) << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod smlsl_za_zzi_4xi {
    #[inline]
    pub fn smlsl_za_zzi_4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i3h.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | u32::from(i3l.into()) << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod umlal_za_zzi_4xi {
    #[inline]
    pub fn umlal_za_zzi_4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i3h.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | u32::from(i3l.into()) << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod umlsl_za_zzi_4xi {
    #[inline]
    pub fn umlsl_za_zzi_4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i3h.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(U.into()) << 4u32
                | u32::from(S.into()) << 3u32
                | u32::from(i3l.into()) << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
