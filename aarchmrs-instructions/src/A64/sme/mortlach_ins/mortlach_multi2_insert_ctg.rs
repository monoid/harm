/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_za2_z_b1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_za2_z_b1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_za2_z_b1 {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { V, Rs, Zn, off3 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000000000100u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | 0b000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b000u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod mova_za2_z_h1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_za2_z_h1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub ZAd: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl mova_za2_z_h1 {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            ZAd: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                V,
                Rs,
                Zn,
                ZAd,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000001000100u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | 0b000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b000u32 << 3u32
                    | self.ZAd.into_inner() << 2u32
                    | self.off2.into_inner() << 0u32,
            )
        }
    }
}
pub mod mova_za2_z_w1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_za2_z_w1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub ZAd: ::aarchmrs_types::BitValue<2>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl mova_za2_z_w1 {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            ZAd: ::aarchmrs_types::BitValue<2>,
            o1: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { V, Rs, Zn, ZAd, o1 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000010000100u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | 0b000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b000u32 << 3u32
                    | self.ZAd.into_inner() << 1u32
                    | self.o1.into_inner() << 0u32,
            )
        }
    }
}
pub mod mova_za2_z_d1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_za2_z_d1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub ZAd: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_za2_z_d1 {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            ZAd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { V, Rs, Zn, ZAd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011000100u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | 0b000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b000u32 << 3u32
                    | self.ZAd.into_inner() << 0u32,
            )
        }
    }
}
