/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBZ_32_compbranch {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBZ_32_compbranch {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBZ_32_compbranch {
        #[inline]
        pub fn new(
            imm19: impl Into<::aarchmrs_types::BitValue<19>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm19: imm19.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00110100u32 << 24u32 | u32::from(self.imm19) << 5u32 | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod CBNZ_32_compbranch {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBNZ_32_compbranch {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBNZ_32_compbranch {
        #[inline]
        pub fn new(
            imm19: impl Into<::aarchmrs_types::BitValue<19>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm19: imm19.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00110101u32 << 24u32 | u32::from(self.imm19) << 5u32 | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod CBZ_64_compbranch {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBZ_64_compbranch {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBZ_64_compbranch {
        #[inline]
        pub fn new(
            imm19: impl Into<::aarchmrs_types::BitValue<19>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm19: imm19.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10110100u32 << 24u32 | u32::from(self.imm19) << 5u32 | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod CBNZ_64_compbranch {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBNZ_64_compbranch {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBNZ_64_compbranch {
        #[inline]
        pub fn new(
            imm19: impl Into<::aarchmrs_types::BitValue<19>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm19: imm19.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10110101u32 << 24u32 | u32::from(self.imm19) << 5u32 | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
