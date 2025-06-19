/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_za_zzi_d2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmla_za_zzi_d2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmla_za_zzi_d2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i1: i1.into(),
                Zn: Zn.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b00u32 << 11u32
                    | u32::from(self.i1) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.S) << 4u32
                    | 0b0u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod sdot_za_zzi_d2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sdot_za_zzi_d2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl sdot_za_zzi_d2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i1: i1.into(),
                Zn: Zn.into(),
                U: U.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b00u32 << 11u32
                    | u32::from(self.i1) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b1u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod fmls_za_zzi_d2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmls_za_zzi_d2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmls_za_zzi_d2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i1: i1.into(),
                Zn: Zn.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b00u32 << 11u32
                    | u32::from(self.i1) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.S) << 4u32
                    | 0b0u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod udot_za_zzi_d2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct udot_za_zzi_d2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl udot_za_zzi_d2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i1: i1.into(),
                Zn: Zn.into(),
                U: U.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b00u32 << 11u32
                    | u32::from(self.i1) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b1u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
