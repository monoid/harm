/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADDG_64_addsub_immtags {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDG_64_addsub_immtags {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDG_64_addsub_immtags {
        #[inline]
        pub fn new(
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm6: imm6.into(),
                imm4: imm4.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1001000110u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.imm4) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SUBG_64_addsub_immtags {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUBG_64_addsub_immtags {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUBG_64_addsub_immtags {
        #[inline]
        pub fn new(
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm6: imm6.into(),
                imm4: imm4.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101000110u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.imm4) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
