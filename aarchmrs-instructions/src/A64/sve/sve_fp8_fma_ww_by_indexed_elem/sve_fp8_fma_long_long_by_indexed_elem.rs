/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlallbb_z32_z8z8z8i_ {
    #[inline]
    pub fn fmlallbb_z32_z8z8z8i_(
        TT: impl Into<::aarchmrs_types::BitValue<2>>,
        i4h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        i4l: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100u32 << 24u32
                | u32::from(TT.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(i4h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b1100u32 << 12u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmlallbt_z32_z8z8z8i_ {
    #[inline]
    pub fn fmlallbt_z32_z8z8z8i_(
        TT: impl Into<::aarchmrs_types::BitValue<2>>,
        i4h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        i4l: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100u32 << 24u32
                | u32::from(TT.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(i4h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b1100u32 << 12u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmlalltb_z32_z8z8z8i_ {
    #[inline]
    pub fn fmlalltb_z32_z8z8z8i_(
        TT: impl Into<::aarchmrs_types::BitValue<2>>,
        i4h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        i4l: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100u32 << 24u32
                | u32::from(TT.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(i4h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b1100u32 << 12u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmlalltt_z32_z8z8z8i_ {
    #[inline]
    pub fn fmlalltt_z32_z8z8z8i_(
        TT: impl Into<::aarchmrs_types::BitValue<2>>,
        i4h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        i4l: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100u32 << 24u32
                | u32::from(TT.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(i4h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b1100u32 << 12u32
                | u32::from(i4l.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
