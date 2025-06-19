/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zero_zt_i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zero_zt_i_ {}
    impl zero_zt_i_ {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000010010000000000000000001u32 << 0u32,
            )
        }
    }
}
