/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod aese_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aese_z_zz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl aese_z_zz_ {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0100010100100010111000u32 << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod aesd_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aesd_z_zz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl aesd_z_zz_ {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0100010100100010111001u32 << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sm4e_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sm4e_z_zz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sm4e_z_zz_ {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0100010100100011111000u32 << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
