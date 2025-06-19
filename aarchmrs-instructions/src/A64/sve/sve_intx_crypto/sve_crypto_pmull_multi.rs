/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pmull_mz_zzw_1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pmull_mz_zzw_1x2 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
    }
    impl pmull_mz_zzw_1x2 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b111110u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
