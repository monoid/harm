/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcvt_z8_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvt_z8_mz4_ {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl fcvt_z8_mz4_ {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                N: N.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100110100111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b0u32 << 6u32
                    | u32::from(self.N) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod fcvtn_z8_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvtn_z8_mz4_ {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl fcvtn_z8_mz4_ {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                N: N.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100110100111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b0u32 << 6u32
                    | u32::from(self.N) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
