/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TSTART_BR_systemresult {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TSTART_BR_systemresult {
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl TSTART_BR_systemresult {
        #[inline]
        pub fn new(Rt: impl Into<::aarchmrs_types::BitValue<5>>) -> Self {
            Self { Rt: Rt.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101010010001100110000011u32 << 5u32 | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod TTEST_BR_systemresult {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TTEST_BR_systemresult {
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl TTEST_BR_systemresult {
        #[inline]
        pub fn new(Rt: impl Into<::aarchmrs_types::BitValue<5>>) -> Self {
            Self { Rt: Rt.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101010010001100110001011u32 << 5u32 | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
