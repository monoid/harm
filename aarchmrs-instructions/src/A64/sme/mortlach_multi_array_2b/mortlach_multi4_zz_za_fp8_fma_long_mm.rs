/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlal_za_z8z8w_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlal_za_z8z8w_4x4 {
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl fmlal_za_z8z8w_4x4 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<3>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<3>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Zm, Rv, Zn, off2 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001101u32 << 21u32
                    | self.Zm.into_inner() << 18u32
                    | 0b010u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b010u32 << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b01000u32 << 2u32
                    | self.off2.into_inner() << 0u32,
            )
        }
    }
}
