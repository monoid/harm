/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfcvt_z8_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfcvt_z8_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl bfcvt_z8_mz2_ {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000101100100111000u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod fcvt_z8_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvt_z8_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl fcvt_z8_mz2_ {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100100100111000u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
