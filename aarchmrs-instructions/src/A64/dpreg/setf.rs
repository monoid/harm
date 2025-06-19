/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SETF8_only_setf {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETF8_only_setf {
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl SETF8_only_setf {
        #[inline]
        pub fn new(Rn: impl Into<::aarchmrs_types::BitValue<5>>) -> Self {
            Self { Rn: Rn.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011101000000000000010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b01101u32 << 0u32,
            )
        }
    }
}
pub mod SETF16_only_setf {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETF16_only_setf {
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl SETF16_only_setf {
        #[inline]
        pub fn new(Rn: impl Into<::aarchmrs_types::BitValue<5>>) -> Self {
            Self { Rn: Rn.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011101000000000010010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b01101u32 << 0u32,
            )
        }
    }
}
