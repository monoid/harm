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
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm2: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                imm2: imm2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.imm2) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
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
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm2: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                imm2: imm2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.imm2) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
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
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm2: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                imm2: imm2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.imm2) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
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
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm2: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                imm2: imm2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.imm2) << 12u32
                    | 0b11u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
