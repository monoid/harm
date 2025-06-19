/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod wrffr_f_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct wrffr_f_p_ {
        pub Pn: ::aarchmrs_types::BitValue<4>,
    }
    impl wrffr_f_p_ {
        #[inline]
        pub const fn new(Pn: ::aarchmrs_types::BitValue<4>) -> Self {
            Self { Pn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101001010001001000u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b00000u32 << 0u32,
            )
        }
    }
}
