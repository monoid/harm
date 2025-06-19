/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqdmullb_z_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdmullb_z_zzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdmullb_z_zzi_s {
        #[inline]
        pub const fn new(
            i3h: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            i3l: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                i3h,
                Zm,
                i3l,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | self.i3h.into_inner() << 19u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1110u32 << 12u32
                    | self.i3l.into_inner() << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqdmullb_z_zzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdmullb_z_zzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdmullb_z_zzi_d {
        #[inline]
        pub const fn new(
            i2h: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<4>,
            i2l: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                i2h,
                Zm,
                i2l,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | self.i2h.into_inner() << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1110u32 << 12u32
                    | self.i2l.into_inner() << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqdmullt_z_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdmullt_z_zzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdmullt_z_zzi_s {
        #[inline]
        pub const fn new(
            i3h: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            i3l: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                i3h,
                Zm,
                i3l,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | self.i3h.into_inner() << 19u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1110u32 << 12u32
                    | self.i3l.into_inner() << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqdmullt_z_zzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdmullt_z_zzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdmullt_z_zzi_d {
        #[inline]
        pub const fn new(
            i2h: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<4>,
            i2l: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                i2h,
                Zm,
                i2l,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | self.i2h.into_inner() << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1110u32 << 12u32
                    | self.i2l.into_inner() << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
