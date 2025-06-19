/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmax_mz_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmax_mz_zzw_2x2 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl fmax_mz_zzw_2x2 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
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
                    | u32::from(self.Zm) << 17u32
                    | 0b010110001000u32 << 5u32
                    | u32::from(self.Zdn) << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod bfmax_mz_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmax_mz_zzw_2x2 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl bfmax_mz_zzw_2x2 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001001u32 << 21u32
                    | u32::from(self.Zm) << 17u32
                    | 0b010110001000u32 << 5u32
                    | u32::from(self.Zdn) << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod fmin_mz_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmin_mz_zzw_2x2 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl fmin_mz_zzw_2x2 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
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
                    | u32::from(self.Zm) << 17u32
                    | 0b010110001000u32 << 5u32
                    | u32::from(self.Zdn) << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
pub mod bfmin_mz_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmin_mz_zzw_2x2 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl bfmin_mz_zzw_2x2 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001001u32 << 21u32
                    | u32::from(self.Zm) << 17u32
                    | 0b010110001000u32 << 5u32
                    | u32::from(self.Zdn) << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
pub mod fmaxnm_mz_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmaxnm_mz_zzw_2x2 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl fmaxnm_mz_zzw_2x2 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
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
                    | u32::from(self.Zm) << 17u32
                    | 0b010110001001u32 << 5u32
                    | u32::from(self.Zdn) << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod bfmaxnm_mz_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmaxnm_mz_zzw_2x2 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl bfmaxnm_mz_zzw_2x2 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001001u32 << 21u32
                    | u32::from(self.Zm) << 17u32
                    | 0b010110001001u32 << 5u32
                    | u32::from(self.Zdn) << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod fminnm_mz_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fminnm_mz_zzw_2x2 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl fminnm_mz_zzw_2x2 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
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
                    | u32::from(self.Zm) << 17u32
                    | 0b010110001001u32 << 5u32
                    | u32::from(self.Zdn) << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
pub mod bfminnm_mz_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfminnm_mz_zzw_2x2 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl bfminnm_mz_zzw_2x2 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001001u32 << 21u32
                    | u32::from(self.Zm) << 17u32
                    | 0b010110001001u32 << 5u32
                    | u32::from(self.Zdn) << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
pub mod famax_mz_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct famax_mz_zzw_2x2 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl famax_mz_zzw_2x2 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
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
                    | u32::from(self.Zm) << 17u32
                    | 0b010110001010u32 << 5u32
                    | u32::from(self.Zdn) << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod famin_mz_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct famin_mz_zzw_2x2 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl famin_mz_zzw_2x2 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<4>>,
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
                    | u32::from(self.Zm) << 17u32
                    | 0b010110001010u32 << 5u32
                    | u32::from(self.Zdn) << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
