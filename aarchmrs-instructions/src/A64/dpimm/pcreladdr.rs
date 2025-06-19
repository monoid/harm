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
        pub const fn new(
            immlo: ::aarchmrs_types::BitValue<2>,
            immhi: ::aarchmrs_types::BitValue<19>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immlo, immhi, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.immlo.into_inner() << 29u32
                    | 0b10000u32 << 24u32
                    | self.immhi.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            immlo: ::aarchmrs_types::BitValue<2>,
            immhi: ::aarchmrs_types::BitValue<19>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immlo, immhi, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1u32 << 31u32
                    | self.immlo.into_inner() << 29u32
                    | 0b10000u32 << 24u32
                    | self.immhi.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
