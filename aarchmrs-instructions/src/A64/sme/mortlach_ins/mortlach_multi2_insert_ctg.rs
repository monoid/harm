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
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                Zn: Zn.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000000000100u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b000u32 << 3u32
                    | u32::from(self.off3) << 0u32,
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
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            ZAd: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                Zn: Zn.into(),
                ZAd: ZAd.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000001000100u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b000u32 << 3u32
                    | u32::from(self.ZAd) << 2u32
                    | u32::from(self.off2) << 0u32,
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
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            ZAd: impl Into<::aarchmrs_types::BitValue<2>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                Zn: Zn.into(),
                ZAd: ZAd.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000010000100u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b000u32 << 3u32
                    | u32::from(self.ZAd) << 1u32
                    | u32::from(self.o1) << 0u32,
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
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            ZAd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                Zn: Zn.into(),
                ZAd: ZAd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011000100u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b000u32 << 3u32
                    | u32::from(self.ZAd) << 0u32,
            )
        }
    }
}
