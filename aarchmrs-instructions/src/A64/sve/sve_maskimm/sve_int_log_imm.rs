/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod orr_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct orr_z_zi_ {
        pub imm13: ::aarchmrs_types::BitValue<13>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl orr_z_zi_ {
        #[inline]
        pub fn new(
            imm13: impl Into<::aarchmrs_types::BitValue<13>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm13: imm13.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101000000u32 << 18u32
                    | u32::from(self.imm13) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod eor_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct eor_z_zi_ {
        pub imm13: ::aarchmrs_types::BitValue<13>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl eor_z_zi_ {
        #[inline]
        pub fn new(
            imm13: impl Into<::aarchmrs_types::BitValue<13>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm13: imm13.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101010000u32 << 18u32
                    | u32::from(self.imm13) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod and_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct and_z_zi_ {
        pub imm13: ::aarchmrs_types::BitValue<13>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl and_z_zi_ {
        #[inline]
        pub fn new(
            imm13: impl Into<::aarchmrs_types::BitValue<13>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm13: imm13.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101100000u32 << 18u32
                    | u32::from(self.imm13) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
