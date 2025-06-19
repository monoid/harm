/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sdot_z32_zzzi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sdot_z32_zzzi_ {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl sdot_z32_zzzi_ {
        #[inline]
        pub fn new(
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2: i2.into(),
                Zm: Zm.into(),
                U: U.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100100u32 << 21u32
                    | u32::from(self.i2) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b11001u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod udot_z32_zzzi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct udot_z32_zzzi_ {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl udot_z32_zzzi_ {
        #[inline]
        pub fn new(
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2: i2.into(),
                Zm: Zm.into(),
                U: U.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100100u32 << 21u32
                    | u32::from(self.i2) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b11001u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
