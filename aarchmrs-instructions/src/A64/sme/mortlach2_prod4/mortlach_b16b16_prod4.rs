/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfmop4a_za_zz_h1x1 {
    #[inline]
    pub fn bfmop4a_za_zz_h1x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b00100u32 << 1u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h1x1 {
    #[inline]
    pub fn bfmop4s_za_zz_h1x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b01100u32 << 1u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod bfmop4a_za_zz_h1x2 {
    #[inline]
    pub fn bfmop4a_za_zz_h1x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b00100u32 << 1u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h1x2 {
    #[inline]
    pub fn bfmop4s_za_zz_h1x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b01100u32 << 1u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod bfmop4a_za_zz_h2x1 {
    #[inline]
    pub fn bfmop4a_za_zz_h2x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b00100u32 << 1u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h2x1 {
    #[inline]
    pub fn bfmop4s_za_zz_h2x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b01100u32 << 1u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod bfmop4a_za_zz_h2x2 {
    #[inline]
    pub fn bfmop4a_za_zz_h2x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b00100u32 << 1u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h2x2 {
    #[inline]
    pub fn bfmop4s_za_zz_h2x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b01100u32 << 1u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
