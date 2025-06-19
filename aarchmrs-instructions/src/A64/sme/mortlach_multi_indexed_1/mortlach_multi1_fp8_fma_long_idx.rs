/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlal_za_z8z8i_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlal_za_z8z8i_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i4A: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i4B: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub i4C: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmlal_za_z8z8i_1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i4A: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i4B: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            i4C: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                i4A,
                Rv,
                i4B,
                Zn,
                i4C,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011100u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i4A.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b0u32 << 12u32
                    | self.i4B.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.i4C.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
