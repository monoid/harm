/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqdmulh_mz_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdmulh_mz_zzw_4x4 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
    }
    impl sqdmulh_mz_zzw_4x4 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Zm) << 18u32
                    | 0b0010111100000u32 << 5u32
                    | u32::from(self.Zdn) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
