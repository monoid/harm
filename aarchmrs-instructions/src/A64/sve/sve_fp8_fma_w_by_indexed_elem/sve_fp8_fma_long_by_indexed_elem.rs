/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlalb_z_z8z8z8i_ {
    #[inline]
    pub fn fmlalb_z_z8z8z8i_(
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        i4h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        i4l: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100u32 << 24u32
                | u32::from(T.into()) << 23u32
                | 0b01u32 << 21u32
                | u32::from(i4h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b0101u32 << 12u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmlalt_z_z8z8z8i_ {
    #[inline]
    pub fn fmlalt_z_z8z8z8i_(
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        i4h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        i4l: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100u32 << 24u32
                | u32::from(T.into()) << 23u32
                | 0b01u32 << 21u32
                | u32::from(i4h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b0101u32 << 12u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
