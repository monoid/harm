/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlall_za32_z8z8i_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlall_za32_z8z8i_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i4h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i4l: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl fmlall_za32_z8z8i_1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i4h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i4l: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i4h,
                Rv,
                i4l,
                Zn,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010100u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i4h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b000u32 << 2u32
                    | self.off2.into_inner() << 0u32,
            )
        }
    }
}
