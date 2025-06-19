/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_za_mz2_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_za_mz2_1 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_za_mz2_1 {
        #[inline]
        pub const fn new(
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Rv, Zn, off3 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000001000u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b010u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b000u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
