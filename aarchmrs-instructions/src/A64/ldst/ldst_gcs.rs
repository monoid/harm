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
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101100100011111000011u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
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
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101100100011111000111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
