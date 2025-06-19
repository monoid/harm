/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smullb_z_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smullb_z_zzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl smullb_z_zzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod smullb_z_zzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smullb_z_zzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl smullb_z_zzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod smullt_z_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smullt_z_zzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl smullt_z_zzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod smullt_z_zzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smullt_z_zzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl smullt_z_zzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod umullb_z_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umullb_z_zzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl umullb_z_zzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod umullb_z_zzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umullb_z_zzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl umullb_z_zzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod umullt_z_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umullt_z_zzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl umullt_z_zzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod umullt_z_zzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umullt_z_zzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl umullt_z_zzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
