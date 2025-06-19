/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcvt_mz2_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvt_mz2_z_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
        pub L: ::aarchmrs_types::BitValue<1>,
    }
    impl fcvt_mz2_z_ {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<4>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                Zd: Zd.into(),
                L: L.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000110100000111000u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 1u32
                    | u32::from(self.L) << 0u32,
            )
        }
    }
}
pub mod fcvtl_mz2_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvtl_mz2_z_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
        pub L: ::aarchmrs_types::BitValue<1>,
    }
    impl fcvtl_mz2_z_ {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<4>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                Zd: Zd.into(),
                L: L.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000110100000111000u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 1u32
                    | u32::from(self.L) << 0u32,
            )
        }
    }
}
