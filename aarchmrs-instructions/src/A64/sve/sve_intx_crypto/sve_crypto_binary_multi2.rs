/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod aese_mz_zzi_2x1 {
    #[inline]
    pub fn aese_mz_zzi_2x1(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001u32 << 21u32
                | u32::from(i2.into()) << 19u32
                | 0b010111010u32 << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod aesd_mz_zzi_2x1 {
    #[inline]
    pub fn aesd_mz_zzi_2x1(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001u32 << 21u32
                | u32::from(i2.into()) << 19u32
                | 0b010111011u32 << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod aesemc_mz_zzi_2x1 {
    #[inline]
    pub fn aesemc_mz_zzi_2x1(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001u32 << 21u32
                | u32::from(i2.into()) << 19u32
                | 0b011111010u32 << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod aesdimc_mz_zzi_2x1 {
    #[inline]
    pub fn aesdimc_mz_zzi_2x1(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001u32 << 21u32
                | u32::from(i2.into()) << 19u32
                | 0b011111011u32 << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
