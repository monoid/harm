/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod XAR_VVV2_crypto3_imm6 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct XAR_VVV2_crypto3_imm6 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl XAR_VVV2_crypto3_imm6 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001110100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
