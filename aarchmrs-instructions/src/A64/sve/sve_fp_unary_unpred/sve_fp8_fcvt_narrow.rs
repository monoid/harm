/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcvtn_z8_mz2_h2b {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvtn_z8_mz2_h2b {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl fcvtn_z8_mz2_h2b {
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
                0b0110010100001010001100u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod fcvtnb_z8_mz2_s2b {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvtnb_z8_mz2_s2b {
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl fcvtnb_z8_mz2_s2b {
        #[inline]
        pub fn new(
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101000010100011u32 << 12u32
                    | u32::from(self.T) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod bfcvtn_z8_mz2_bf2b {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfcvtn_z8_mz2_bf2b {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl bfcvtn_z8_mz2_bf2b {
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
                0b0110010100001010001110u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod fcvtnt_z8_mz2_s2b {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvtnt_z8_mz2_s2b {
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl fcvtnt_z8_mz2_s2b {
        #[inline]
        pub fn new(
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101000010100011u32 << 12u32
                    | u32::from(self.T) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
