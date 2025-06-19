/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptrue_pn_i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ptrue_pn_i_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl ptrue_pn_i_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            PNd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { size, PNd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1000000111100000010u32 << 3u32
                    | self.PNd.into_inner() << 0u32,
            )
        }
    }
}
