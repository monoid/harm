/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlall_za32_z8z8v_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlall_za32_z8z8v_2x1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl fmlall_za32_z8z8v_2x1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            o1: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { Zm, Rv, Zn, o1 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b000u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b0001u32 << 1u32
                    | self.o1.into_inner() << 0u32,
            )
        }
    }
}
