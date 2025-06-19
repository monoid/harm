/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FMOV_S_floatimm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_S_floatimm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_S_floatimm {
        #[inline]
        pub fn new(
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm8: imm8.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | u32::from(self.imm8) << 13u32
                    | 0b10000000u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMOV_D_floatimm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_D_floatimm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_D_floatimm {
        #[inline]
        pub fn new(
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm8: imm8.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | u32::from(self.imm8) << 13u32
                    | 0b10000000u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMOV_H_floatimm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_H_floatimm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_H_floatimm {
        #[inline]
        pub fn new(
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm8: imm8.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | u32::from(self.imm8) << 13u32
                    | 0b10000000u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
