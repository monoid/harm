/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcvtzs_mz_z_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvtzs_mz_z_4 {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl fcvtzs_mz_z_4 {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100110001111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b0u32 << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod fcvtzu_mz_z_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvtzu_mz_z_4 {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl fcvtzu_mz_z_4 {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100110001111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b0u32 << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
