/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod setffr_f_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct setffr_f_ {}
    impl setffr_f_ {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101001011001001000000000000u32 << 0u32,
            )
        }
    }
}
