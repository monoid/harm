/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fvdotb_za32_z8z8i_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fvdotb_za32_z8z8i_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fvdotb_za32_z8z8i_2xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i2h: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<4>,
            T: ::aarchmrs_types::BitValue<1>,
            i2l: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i2h,
                Zn,
                T,
                i2l,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b01u32 << 11u32
                    | self.i2h.into_inner() << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.T.into_inner() << 4u32
                    | self.i2l.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod fvdott_za32_z8z8i_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fvdott_za32_z8z8i_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2h: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub i2l: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fvdott_za32_z8z8i_2xi {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i2h: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<4>,
            T: ::aarchmrs_types::BitValue<1>,
            i2l: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Rv,
                i2h,
                Zn,
                T,
                i2l,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011101u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b01u32 << 11u32
                    | self.i2h.into_inner() << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.T.into_inner() << 4u32
                    | self.i2l.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
