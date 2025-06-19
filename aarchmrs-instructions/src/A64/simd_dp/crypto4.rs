/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod EOR3_VVV16_crypto4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EOR3_VVV16_crypto4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EOR3_VVV16_crypto4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Ra.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod BCAX_VVV16_crypto4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BCAX_VVV16_crypto4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BCAX_VVV16_crypto4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Ra.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SM3SS1_VVV4_crypto4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SM3SS1_VVV4_crypto4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SM3SS1_VVV4_crypto4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Ra.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
