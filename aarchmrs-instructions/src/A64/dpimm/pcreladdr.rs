/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADR_only_pcreladdr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADR_only_pcreladdr {
        pub immlo: ::aarchmrs_types::BitValue<2>,
        pub immhi: ::aarchmrs_types::BitValue<19>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADR_only_pcreladdr {
        #[inline]
        pub fn new(
            immlo: impl Into<::aarchmrs_types::BitValue<2>>,
            immhi: impl Into<::aarchmrs_types::BitValue<19>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immlo: immlo.into(),
                immhi: immhi.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.immlo) << 29u32
                    | 0b10000u32 << 24u32
                    | u32::from(self.immhi) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ADRP_only_pcreladdr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADRP_only_pcreladdr {
        pub immlo: ::aarchmrs_types::BitValue<2>,
        pub immhi: ::aarchmrs_types::BitValue<19>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADRP_only_pcreladdr {
        #[inline]
        pub fn new(
            immlo: impl Into<::aarchmrs_types::BitValue<2>>,
            immhi: impl Into<::aarchmrs_types::BitValue<19>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immlo: immlo.into(),
                immhi: immhi.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1u32 << 31u32
                    | u32::from(self.immlo) << 29u32
                    | 0b10000u32 << 24u32
                    | u32::from(self.immhi) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
