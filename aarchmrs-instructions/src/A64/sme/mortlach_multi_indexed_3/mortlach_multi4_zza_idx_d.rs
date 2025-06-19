/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_za_zzi_d4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmla_za_zzi_d4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmla_za_zzi_d4xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i1: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i1,
                Zn,
                S,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b00u32 << 11u32
                    | self.i1.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b00u32 << 5u32
                    | self.S.into_inner() << 4u32
                    | 0b0u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod sdot_za_zzi_d4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sdot_za_zzi_d4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl sdot_za_zzi_d4xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i1: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i1,
                Zn,
                U,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b00u32 << 11u32
                    | self.i1.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b00u32 << 5u32
                    | self.U.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod svdot_za_zzi_d4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct svdot_za_zzi_d4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl svdot_za_zzi_d4xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i1: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i1,
                Zn,
                U,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b01u32 << 11u32
                    | self.i1.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b00u32 << 5u32
                    | self.U.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmls_za_zzi_d4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmls_za_zzi_d4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmls_za_zzi_d4xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i1: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i1,
                Zn,
                S,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b00u32 << 11u32
                    | self.i1.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b00u32 << 5u32
                    | self.S.into_inner() << 4u32
                    | 0b0u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod udot_za_zzi_d4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct udot_za_zzi_d4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl udot_za_zzi_d4xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i1: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i1,
                Zn,
                U,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b00u32 << 11u32
                    | self.i1.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b00u32 << 5u32
                    | self.U.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod uvdot_za_zzi_d4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uvdot_za_zzi_d4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl uvdot_za_zzi_d4xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i1: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i1,
                Zn,
                U,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b01u32 << 11u32
                    | self.i1.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b00u32 << 5u32
                    | self.U.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
