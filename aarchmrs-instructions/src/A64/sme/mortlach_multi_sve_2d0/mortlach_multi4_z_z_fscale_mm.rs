/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fscale_mz_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fscale_mz_zzw_4x4 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
    }
    impl fscale_mz_zzw_4x4 {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { size, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 18u32
                    | 0b0010111001100u32 << 5u32
                    | self.Zdn.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod bfscale_mz_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfscale_mz_zzw_4x4 {
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
    }
    impl bfscale_mz_zzw_4x4 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001001u32 << 21u32
                    | self.Zm.into_inner() << 18u32
                    | 0b0010111001100u32 << 5u32
                    | self.Zdn.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
