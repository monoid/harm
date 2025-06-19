/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SM3TT1A_VVV4_crypto3_imm2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SM3TT1A_VVV4_crypto3_imm2 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm2: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SM3TT1A_VVV4_crypto3_imm2 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm2: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm2, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.imm2.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SM3TT1B_VVV4_crypto3_imm2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SM3TT1B_VVV4_crypto3_imm2 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm2: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SM3TT1B_VVV4_crypto3_imm2 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm2: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm2, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.imm2.into_inner() << 12u32
                    | 0b01u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SM3TT2A_VVV4_crypto3_imm2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SM3TT2A_VVV4_crypto3_imm2 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm2: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SM3TT2A_VVV4_crypto3_imm2 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm2: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm2, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.imm2.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SM3TT2B_VVV_crypto3_imm2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SM3TT2B_VVV_crypto3_imm2 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm2: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SM3TT2B_VVV_crypto3_imm2 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm2: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm2, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.imm2.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
