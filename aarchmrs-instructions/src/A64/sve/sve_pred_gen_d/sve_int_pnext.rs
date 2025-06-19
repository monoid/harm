/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pnext_p_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pnext_p_p_p_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pv: ::aarchmrs_types::BitValue<4>,
        pub Pdn: ::aarchmrs_types::BitValue<4>,
    }
    impl pnext_p_p_p_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pv: ::aarchmrs_types::BitValue<4>,
            Pdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { size, Pv, Pdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b0110011100010u32 << 9u32
                    | self.Pv.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pdn.into_inner() << 0u32,
            )
        }
    }
}
