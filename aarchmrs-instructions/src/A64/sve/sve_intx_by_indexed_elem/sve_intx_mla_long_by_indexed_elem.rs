/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlalb_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlalb_z_zzzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl smlalb_z_zzzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod smlalb_z_zzzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlalb_z_zzzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl smlalb_z_zzzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod smlslb_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlslb_z_zzzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl smlslb_z_zzzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod smlslb_z_zzzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlslb_z_zzzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl smlslb_z_zzzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod smlalt_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlalt_z_zzzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl smlalt_z_zzzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod smlalt_z_zzzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlalt_z_zzzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl smlalt_z_zzzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod smlslt_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlslt_z_zzzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl smlslt_z_zzzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod smlslt_z_zzzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlslt_z_zzzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl smlslt_z_zzzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod umlalb_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlalb_z_zzzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl umlalb_z_zzzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod umlalb_z_zzzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlalb_z_zzzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl umlalb_z_zzzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod umlslb_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlslb_z_zzzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl umlslb_z_zzzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod umlslb_z_zzzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlslb_z_zzzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl umlslb_z_zzzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod umlalt_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlalt_z_zzzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl umlalt_z_zzzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod umlalt_z_zzzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlalt_z_zzzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl umlalt_z_zzzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod umlslt_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlslt_z_zzzi_s {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl umlslt_z_zzzi_s {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i3l: i3l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | u32::from(self.i3h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i3l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod umlslt_z_zzzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlslt_z_zzzi_d {
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl umlslt_z_zzzi_d {
        #[inline]
        pub fn new(
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2h: i2h.into(),
                Zm: Zm.into(),
                S: S.into(),
                U: U.into(),
                i2l: i2l.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | u32::from(self.i2h) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | u32::from(self.U) << 12u32
                    | u32::from(self.i2l) << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
