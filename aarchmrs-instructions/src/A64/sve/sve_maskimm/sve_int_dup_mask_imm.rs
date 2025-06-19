/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dupm_z_i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct dupm_z_i_ {
        pub imm13: ::aarchmrs_types::BitValue<13>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl dupm_z_i_ {
        #[inline]
        pub fn new(
            imm13: impl Into<::aarchmrs_types::BitValue<13>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm13: imm13.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101110000u32 << 18u32
                    | u32::from(self.imm13) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
