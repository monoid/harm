/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dupq_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct dupq_z_zi_ {
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub tsz: ::aarchmrs_types::BitValue<4>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl dupq_z_zi_ {
        #[inline]
        pub fn new(
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            tsz: impl Into<::aarchmrs_types::BitValue<4>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i1: i1.into(),
                tsz: tsz.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101001u32 << 21u32
                    | u32::from(self.i1) << 20u32
                    | u32::from(self.tsz) << 16u32
                    | 0b001001u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
