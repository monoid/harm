/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mlapt_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mlapt_z_zzz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl mlapt_z_zzz_ {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100110u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110100u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod madpt_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct madpt_z_zzz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl madpt_z_zzz_ {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Za: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Za: Za.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100110u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110110u32 << 10u32
                    | u32::from(self.Za) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
