/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqdmlalb_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdmlalb_z_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdmlalb_z_zzz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                Zm,
                T,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b01100u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqdmlslb_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdmlslb_z_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdmlslb_z_zzz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                Zm,
                T,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b01101u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqdmlalt_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdmlalt_z_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdmlalt_z_zzz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                Zm,
                T,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b01100u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqdmlslt_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdmlslt_z_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdmlslt_z_zzz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                Zm,
                T,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b01101u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
