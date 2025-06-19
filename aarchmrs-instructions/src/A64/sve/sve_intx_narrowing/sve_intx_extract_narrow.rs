/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqxtnb_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqxtnb_z_zz_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqxtnb_z_zz_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | 0b0000100u32 << 12u32
                    | u32::from(self.U) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqxtunb_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqxtunb_z_zz_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqxtunb_z_zz_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | 0b00001010u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqxtnt_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqxtnt_z_zz_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqxtnt_z_zz_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | 0b0000100u32 << 12u32
                    | u32::from(self.U) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqxtunt_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqxtunt_z_zz_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqxtunt_z_zz_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | 0b00001010u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqxtnb_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqxtnb_z_zz_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqxtnb_z_zz_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | 0b0000100u32 << 12u32
                    | u32::from(self.U) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqxtnt_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqxtnt_z_zz_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqxtnt_z_zz_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | 0b0000100u32 << 12u32
                    | u32::from(self.U) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
