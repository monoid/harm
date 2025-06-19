/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod index_z_ir_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct index_z_ir_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl index_z_ir_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                imm5: imm5.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010010u32 << 10u32
                    | u32::from(self.imm5) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
