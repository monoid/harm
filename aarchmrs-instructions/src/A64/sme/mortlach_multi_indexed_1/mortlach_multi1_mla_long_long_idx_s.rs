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
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i4h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i4l: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            S: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i4h,
                Rv,
                i4l,
                Zn,
                U,
                S,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i4h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.U.into_inner() << 4u32
                    | self.S.into_inner() << 3u32
                    | 0b0u32 << 2u32
                    | self.off2.into_inner() << 0u32,
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
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i4h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i4l: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i4h,
                Rv,
                i4l,
                Zn,
                U,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i4h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.U.into_inner() << 4u32
                    | 0b01u32 << 2u32
                    | self.off2.into_inner() << 0u32,
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
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i4h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i4l: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            S: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i4h,
                Rv,
                i4l,
                Zn,
                U,
                S,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i4h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.U.into_inner() << 4u32
                    | self.S.into_inner() << 3u32
                    | 0b0u32 << 2u32
                    | self.off2.into_inner() << 0u32,
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
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i4h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i4l: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            S: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i4h,
                Rv,
                i4l,
                Zn,
                U,
                S,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i4h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.U.into_inner() << 4u32
                    | self.S.into_inner() << 3u32
                    | 0b0u32 << 2u32
                    | self.off2.into_inner() << 0u32,
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
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i4h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i4l: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i4h,
                Rv,
                i4l,
                Zn,
                U,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i4h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.U.into_inner() << 4u32
                    | 0b01u32 << 2u32
                    | self.off2.into_inner() << 0u32,
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
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i4h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i4l: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            S: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i4h,
                Rv,
                i4l,
                Zn,
                U,
                S,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010000u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i4h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.U.into_inner() << 4u32
                    | self.S.into_inner() << 3u32
                    | 0b0u32 << 2u32
                    | self.off2.into_inner() << 0u32,
            )
        }
    }
}
