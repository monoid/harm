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
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2h: i2h.into(),
                Zn: Zn.into(),
                T: T.into(),
                i2l: i2l.into(),
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
                    | 0b01u32 << 11u32
                    | u32::from(self.i2h) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.T) << 4u32
                    | u32::from(self.i2l) << 3u32
                    | u32::from(self.off3) << 0u32,
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
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2h: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            i2l: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2h: i2h.into(),
                Zn: Zn.into(),
                T: T.into(),
                i2l: i2l.into(),
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
                    | 0b01u32 << 11u32
                    | u32::from(self.i2h) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.T) << 4u32
                    | u32::from(self.i2l) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
