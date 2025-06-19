/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfmlal_za_zzi_4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmlal_za_zzi_4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl bfmlal_za_zzi_4xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i3h: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            i3l: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i3h,
                Zn,
                S,
                i3l,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011001u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b1u32 << 12u32
                    | self.i3h.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b001u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.i3l.into_inner() << 2u32
                    | self.off2.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmlal_za_zzi_4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlal_za_zzi_4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl fmlal_za_zzi_4xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i3h: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            i3l: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i3h,
                Zn,
                S,
                i3l,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011001u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b1u32 << 12u32
                    | self.i3h.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b000u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.i3l.into_inner() << 2u32
                    | self.off2.into_inner() << 0u32,
            )
        }
    }
}
pub mod bfmlsl_za_zzi_4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmlsl_za_zzi_4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl bfmlsl_za_zzi_4xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i3h: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            i3l: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i3h,
                Zn,
                S,
                i3l,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011001u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b1u32 << 12u32
                    | self.i3h.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b001u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.i3l.into_inner() << 2u32
                    | self.off2.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmlsl_za_zzi_4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlsl_za_zzi_4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl fmlsl_za_zzi_4xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i3h: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            i3l: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i3h,
                Zn,
                S,
                i3l,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011001u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b1u32 << 12u32
                    | self.i3h.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b000u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.i3l.into_inner() << 2u32
                    | self.off2.into_inner() << 0u32,
            )
        }
    }
}
