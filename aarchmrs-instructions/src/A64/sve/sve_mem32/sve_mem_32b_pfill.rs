/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldr_p_bi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldr_p_bi_ {
        pub imm9h: ::aarchmrs_types::BitValue<6>,
        pub imm9l: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pt: ::aarchmrs_types::BitValue<4>,
    }
    impl ldr_p_bi_ {
        #[inline]
        pub fn new(
            imm9h: impl Into<::aarchmrs_types::BitValue<6>>,
            imm9l: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pt: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                imm9h: imm9h.into(),
                imm9l: imm9l.into(),
                Rn: Rn.into(),
                Pt: Pt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010110u32 << 22u32
                    | u32::from(self.imm9h) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.imm9l) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pt) << 0u32,
            )
        }
    }
}
