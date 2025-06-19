/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ext_z_zi_des {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ext_z_zi_des {
        pub imm8h: ::aarchmrs_types::BitValue<5>,
        pub imm8l: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl ext_z_zi_des {
        #[inline]
        pub fn new(
            imm8h: impl Into<::aarchmrs_types::BitValue<5>>,
            imm8l: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm8h: imm8h.into(),
                imm8l: imm8l.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101001u32 << 21u32
                    | u32::from(self.imm8h) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.imm8l) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
