/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_za_zzi_h4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmla_za_zzi_h4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmla_za_zzi_h4xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i3h: i3h.into(),
                Zn: Zn.into(),
                S: S.into(),
                i3l: i3l.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010001u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i3h) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b00u32 << 5u32
                    | u32::from(self.S) << 4u32
                    | u32::from(self.i3l) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod bfmla_za_zzi_h4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmla_za_zzi_h4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfmla_za_zzi_h4xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i3h: i3h.into(),
                Zn: Zn.into(),
                S: S.into(),
                i3l: i3l.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010001u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i3h) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b01u32 << 5u32
                    | u32::from(self.S) << 4u32
                    | u32::from(self.i3l) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod fmls_za_zzi_h4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmls_za_zzi_h4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmls_za_zzi_h4xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i3h: i3h.into(),
                Zn: Zn.into(),
                S: S.into(),
                i3l: i3l.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010001u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i3h) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b00u32 << 5u32
                    | u32::from(self.S) << 4u32
                    | u32::from(self.i3l) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod bfmls_za_zzi_h4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmls_za_zzi_h4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfmls_za_zzi_h4xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i3h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i3h: i3h.into(),
                Zn: Zn.into(),
                S: S.into(),
                i3l: i3l.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010001u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i3h) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b01u32 << 5u32
                    | u32::from(self.S) << 4u32
                    | u32::from(self.i3l) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
