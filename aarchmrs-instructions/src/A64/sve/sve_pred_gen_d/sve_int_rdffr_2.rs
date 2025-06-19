/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod rdffr_p_f_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct rdffr_p_f_ {
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl rdffr_p_f_ {
        #[inline]
        pub fn new(Pd: impl Into<::aarchmrs_types::BitValue<4>>) -> Self {
            Self { Pd: Pd.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010010100011001111100000000u32 << 4u32 | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
