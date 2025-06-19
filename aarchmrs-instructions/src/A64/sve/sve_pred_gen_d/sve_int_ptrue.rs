/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptrue_p_s_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ptrue_p_s_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl ptrue_p_s_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            S: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                S,
                pattern,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b01100u32 << 17u32
                    | self.S.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod ptrues_p_s_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ptrues_p_s_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl ptrues_p_s_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            S: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                S,
                pattern,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b01100u32 << 17u32
                    | self.S.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
