/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmax_mz_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmax_mz_zzv_2x1 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl fmax_mz_zzv_2x1 {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { size, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10100001000u32 << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod bfmax_mz_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmax_mz_zzv_2x1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl bfmax_mz_zzv_2x1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10100001000u32 << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod fmin_mz_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmin_mz_zzv_2x1 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl fmin_mz_zzv_2x1 {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { size, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10100001000u32 << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
pub mod bfmin_mz_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmin_mz_zzv_2x1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl bfmin_mz_zzv_2x1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10100001000u32 << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
pub mod fmaxnm_mz_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmaxnm_mz_zzv_2x1 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl fmaxnm_mz_zzv_2x1 {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { size, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10100001001u32 << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod bfmaxnm_mz_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmaxnm_mz_zzv_2x1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl bfmaxnm_mz_zzv_2x1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10100001001u32 << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod fminnm_mz_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fminnm_mz_zzv_2x1 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl fminnm_mz_zzv_2x1 {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { size, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10100001001u32 << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
pub mod bfminnm_mz_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfminnm_mz_zzv_2x1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl bfminnm_mz_zzv_2x1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10100001001u32 << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
