/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sunpklo_z_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sunpklo_z_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sunpklo_z_z_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                H: H.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1100u32 << 18u32
                    | u32::from(self.U) << 17u32
                    | u32::from(self.H) << 16u32
                    | 0b001110u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sunpkhi_z_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sunpkhi_z_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sunpkhi_z_z_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                H: H.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1100u32 << 18u32
                    | u32::from(self.U) << 17u32
                    | u32::from(self.H) << 16u32
                    | 0b001110u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uunpklo_z_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uunpklo_z_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uunpklo_z_z_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                H: H.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1100u32 << 18u32
                    | u32::from(self.U) << 17u32
                    | u32::from(self.H) << 16u32
                    | 0b001110u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uunpkhi_z_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uunpkhi_z_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uunpkhi_z_z_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                H: H.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1100u32 << 18u32
                    | u32::from(self.U) << 17u32
                    | u32::from(self.H) << 16u32
                    | 0b001110u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
