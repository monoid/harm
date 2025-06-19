/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_z_zz_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti2_z_zz_16 {
        pub i3h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub i3l: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl luti2_z_zz_16 {
        #[inline]
        pub const fn new(
            i3h: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            i3l: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                i3h,
                Zm,
                i3l,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101u32 << 24u32
                    | self.i3h.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.i3l.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
