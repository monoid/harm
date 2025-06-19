/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dup_z_r_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct dup_z_r_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl dup_z_r_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rn: Rn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b100000001110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
