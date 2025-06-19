/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ext_z_zi_con {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ext_z_zi_con {
        pub imm8h: ::aarchmrs_types::BitValue<5>,
        pub imm8l: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl ext_z_zi_con {
        #[inline]
        pub fn new(
            imm8h: impl Into<::aarchmrs_types::BitValue<5>>,
            imm8l: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm8h: imm8h.into(),
                imm8l: imm8l.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101011u32 << 21u32
                    | u32::from(self.imm8h) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.imm8l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
