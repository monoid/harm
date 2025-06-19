/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod WFET_only_systeminstrswithreg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct WFET_only_systeminstrswithreg {
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl WFET_only_systeminstrswithreg {
        #[inline]
        pub const fn new(Rd: ::aarchmrs_types::BitValue<5>) -> Self {
            Self { Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101010000001100010000000u32 << 5u32 | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod WFIT_only_systeminstrswithreg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct WFIT_only_systeminstrswithreg {
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl WFIT_only_systeminstrswithreg {
        #[inline]
        pub const fn new(Rd: ::aarchmrs_types::BitValue<5>) -> Self {
            Self { Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101010000001100010000001u32 << 5u32 | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
