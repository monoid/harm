/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fscale_mz_zzv_4x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fscale_mz_zzv_4x1 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
    }
    impl fscale_mz_zzv_4x1 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
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
                    | 0b10u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10101001100u32 << 5u32
                    | u32::from(self.Zdn) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod bfscale_mz_zzv_4x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfscale_mz_zzv_4x1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
    }
    impl bfscale_mz_zzv_4x1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10101001100u32 << 5u32
                    | u32::from(self.Zdn) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
