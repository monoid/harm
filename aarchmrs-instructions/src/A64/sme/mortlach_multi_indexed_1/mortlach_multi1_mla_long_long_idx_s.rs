/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlall_za_zzi_s {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i4h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i4l: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl smlall_za_zzi_s {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            i4h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i4l: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                i4h: i4h.into(),
                Rv: Rv.into(),
                i4l: i4l.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.i4h) << 15u32
                    | u32::from(self.Rv) << 13u32
                    | u32::from(self.i4l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod usmlall_za_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmlall_za_zzi_s {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i4h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i4l: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl usmlall_za_zzi_s {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            i4h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i4l: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                i4h: i4h.into(),
                Rv: Rv.into(),
                i4l: i4l.into(),
                Zn: Zn.into(),
                U: U.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.i4h) << 15u32
                    | u32::from(self.Rv) << 13u32
                    | u32::from(self.i4l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b01u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod smlsll_za_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlsll_za_zzi_s {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i4h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i4l: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl smlsll_za_zzi_s {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            i4h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i4l: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                i4h: i4h.into(),
                Rv: Rv.into(),
                i4l: i4l.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.i4h) << 15u32
                    | u32::from(self.Rv) << 13u32
                    | u32::from(self.i4l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod umlall_za_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlall_za_zzi_s {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i4h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i4l: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl umlall_za_zzi_s {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            i4h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i4l: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                i4h: i4h.into(),
                Rv: Rv.into(),
                i4l: i4l.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.i4h) << 15u32
                    | u32::from(self.Rv) << 13u32
                    | u32::from(self.i4l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod sumlall_za_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumlall_za_zzi_s {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i4h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i4l: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl sumlall_za_zzi_s {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            i4h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i4l: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                i4h: i4h.into(),
                Rv: Rv.into(),
                i4l: i4l.into(),
                Zn: Zn.into(),
                U: U.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.i4h) << 15u32
                    | u32::from(self.Rv) << 13u32
                    | u32::from(self.i4l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b01u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod umlsll_za_zzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlsll_za_zzi_s {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i4h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i4l: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl umlsll_za_zzi_s {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            i4h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i4l: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                i4h: i4h.into(),
                Rv: Rv.into(),
                i4l: i4l.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.i4h) << 15u32
                    | u32::from(self.Rv) << 13u32
                    | u32::from(self.i4l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
