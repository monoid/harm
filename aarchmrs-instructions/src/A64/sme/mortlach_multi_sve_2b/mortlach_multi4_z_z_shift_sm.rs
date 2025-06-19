/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod srshl_mz_zzv_4x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct srshl_mz_zzv_4x1 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
    }
    impl srshl_mz_zzv_4x1 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
                U: U.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10101010001u32 << 5u32
                    | u32::from(self.Zdn) << 2u32
                    | 0b0u32 << 1u32
                    | u32::from(self.U) << 0u32,
            )
        }
    }
}
pub mod urshl_mz_zzv_4x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct urshl_mz_zzv_4x1 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
    }
    impl urshl_mz_zzv_4x1 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
                U: U.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10101010001u32 << 5u32
                    | u32::from(self.Zdn) << 2u32
                    | 0b0u32 << 1u32
                    | u32::from(self.U) << 0u32,
            )
        }
    }
}
