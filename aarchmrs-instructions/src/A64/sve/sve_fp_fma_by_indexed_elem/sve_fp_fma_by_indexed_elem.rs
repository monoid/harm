/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_z_zzzi_h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmla_z_zzzi_h {
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmla_z_zzzi_h {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                i3l: i3l.into(),
                Zm: Zm.into(),
                op: op.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001000u32 << 23u32
                    | u32::from(self.i3h) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.i3l) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b00000u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod bfmla_z_zzzi_h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmla_z_zzzi_h {
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl bfmla_z_zzzi_h {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                i3l: i3l.into(),
                Zm: Zm.into(),
                op: op.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001000u32 << 23u32
                    | u32::from(self.i3h) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.i3l) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b00001u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmla_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmla_z_zzzi_s {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmla_z_zzzi_s {
        #[inline]
        pub fn new(
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2: i2.into(),
                Zm: Zm.into(),
                op: op.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100101u32 << 21u32
                    | u32::from(self.i2) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b00000u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmla_z_zzzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmla_z_zzzi_d {
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmla_z_zzzi_d {
        #[inline]
        pub fn new(
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i1: i1.into(),
                Zm: Zm.into(),
                op: op.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100111u32 << 21u32
                    | u32::from(self.i1) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b00000u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmls_z_zzzi_h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmls_z_zzzi_h {
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmls_z_zzzi_h {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                i3l: i3l.into(),
                Zm: Zm.into(),
                op: op.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001000u32 << 23u32
                    | u32::from(self.i3h) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.i3l) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b00000u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod bfmls_z_zzzi_h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmls_z_zzzi_h {
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl bfmls_z_zzzi_h {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                i3l: i3l.into(),
                Zm: Zm.into(),
                op: op.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001000u32 << 23u32
                    | u32::from(self.i3h) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.i3l) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b00001u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmls_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmls_z_zzzi_s {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmls_z_zzzi_s {
        #[inline]
        pub fn new(
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2: i2.into(),
                Zm: Zm.into(),
                op: op.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100101u32 << 21u32
                    | u32::from(self.i2) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b00000u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmls_z_zzzi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmls_z_zzzi_d {
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmls_z_zzzi_d {
        #[inline]
        pub fn new(
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i1: i1.into(),
                Zm: Zm.into(),
                op: op.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100111u32 << 21u32
                    | u32::from(self.i1) << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b00000u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
