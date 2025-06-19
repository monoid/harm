/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlall_za32_z8z8i_4xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlall_za32_z8z8i_4xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i4h: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub i4l: ::aarchmrs_types::BitValue<2>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl fmlall_za32_z8z8i_4xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i4h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            i4l: impl Into<::aarchmrs_types::BitValue<2>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i4h: i4h.into(),
                Zn: Zn.into(),
                i4l: i4l.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010001u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0u32 << 12u32
                    | u32::from(self.i4h) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b1000u32 << 3u32
                    | u32::from(self.i4l) << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
