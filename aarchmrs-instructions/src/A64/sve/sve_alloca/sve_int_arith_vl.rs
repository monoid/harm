/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod addvl_r_ri_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addvl_r_ri_ {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl addvl_r_ri_ {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                imm6: imm6.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100001u32 << 21u32
                    | u32::from(self.Rn) << 16u32
                    | 0b01010u32 << 11u32
                    | u32::from(self.imm6) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod addpl_r_ri_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addpl_r_ri_ {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl addpl_r_ri_ {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                imm6: imm6.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100011u32 << 21u32
                    | u32::from(self.Rn) << 16u32
                    | 0b01010u32 << 11u32
                    | u32::from(self.imm6) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
