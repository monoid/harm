/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod GCSSTR_64_ldst_gcs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct GCSSTR_64_ldst_gcs {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl GCSSTR_64_ldst_gcs {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101100100011111000011u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod GCSSTTR_64_ldst_gcs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct GCSSTTR_64_ldst_gcs {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl GCSSTTR_64_ldst_gcs {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101100100011111000111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
