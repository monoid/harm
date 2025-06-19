/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqcvtn_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqcvtn_z_mz2_ {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqcvtn_z_mz2_ {
        #[inline]
        pub fn new(
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                U: U.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001100010100u32 << 12u32
                    | u32::from(self.U) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqcvtun_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqcvtun_z_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqcvtun_z_mz2_ {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0100010100110001010100u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqcvtn_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqcvtn_z_mz2_ {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqcvtn_z_mz2_ {
        #[inline]
        pub fn new(
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                U: U.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001100010100u32 << 12u32
                    | u32::from(self.U) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
