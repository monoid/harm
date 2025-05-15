/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlallbb_z32_z8z8z8_ {
    #[inline]
    pub fn fmlallbb_z32_z8z8z8_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        TT: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(TT.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmlallbt_z32_z8z8z8_ {
    #[inline]
    pub fn fmlallbt_z32_z8z8z8_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        TT: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(TT.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmlalltb_z32_z8z8z8_ {
    #[inline]
    pub fn fmlalltb_z32_z8z8z8_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        TT: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(TT.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmlalltt_z32_z8z8z8_ {
    #[inline]
    pub fn fmlalltt_z32_z8z8z8_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        TT: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(TT.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
