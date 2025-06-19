/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_mz4_za_b1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_mz4_za_b1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub off2: ::aarchmrs_types::BitValue<2>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_mz4_za_b1 {
        #[inline]
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                off2: off2.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000000000110u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b001000u32 << 7u32
                    | u32::from(self.off2) << 5u32
                    | u32::from(self.Zd) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod mova_mz4_za_h1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_mz4_za_h1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub ZAn: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_mz4_za_h1 {
        #[inline]
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            ZAn: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                ZAn: ZAn.into(),
                o1: o1.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000001000110u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b001000u32 << 7u32
                    | u32::from(self.ZAn) << 6u32
                    | u32::from(self.o1) << 5u32
                    | u32::from(self.Zd) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod mova_mz4_za_w1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_mz4_za_w1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub ZAn: ::aarchmrs_types::BitValue<2>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_mz4_za_w1 {
        #[inline]
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            ZAn: impl Into<::aarchmrs_types::BitValue<2>>,
            Zd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                ZAn: ZAn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000010000110u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b001000u32 << 7u32
                    | u32::from(self.ZAn) << 5u32
                    | u32::from(self.Zd) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod mova_mz4_za_d1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_mz4_za_d1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub ZAn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_mz4_za_d1 {
        #[inline]
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            ZAn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                ZAn: ZAn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011000110u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b00100u32 << 8u32
                    | u32::from(self.ZAn) << 5u32
                    | u32::from(self.Zd) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
