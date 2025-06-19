/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshr_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshr_z_mz2_ {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshr_z_mz2_ {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                Zn: Zn.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011110u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b110101u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqrshru_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshru_z_mz2_ {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshru_z_mz2_ {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011111u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b110101u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqrshr_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqrshr_z_mz2_ {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqrshr_z_mz2_ {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                Zn: Zn.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011110u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b110101u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
