/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqdmullb_z_zzi_s {
    #[inline]
    pub fn sqdmullb_z_zzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqdmullb_z_zzi_d {
    #[inline]
    pub fn sqdmullb_z_zzi_d(
        i2h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        i2l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i2h.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(i2l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqdmullt_z_zzi_s {
    #[inline]
    pub fn sqdmullt_z_zzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqdmullt_z_zzi_d {
    #[inline]
    pub fn sqdmullt_z_zzi_d(
        i2h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        i2l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i2h.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(i2l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
