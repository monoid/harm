/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RMIF_only_rmif {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RMIF_only_rmif {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub mask: ::aarchmrs_types::BitValue<4>,
    }
    impl RMIF_only_rmif {
        #[inline]
        pub fn new(
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            mask: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                imm6: imm6.into(),
                Rn: Rn.into(),
                mask: mask.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111010000u32 << 21u32
                    | u32::from(self.imm6) << 15u32
                    | 0b00001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.mask) << 0u32,
            )
        }
    }
}
