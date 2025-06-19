/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zip_mz_z_4q {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zip_mz_z_4q {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl zip_mz_z_4q {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100110111111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b00u32 << 5u32
                    | u32::from(self.Zd) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod uzp_mz_z_4q {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uzp_mz_z_4q {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl uzp_mz_z_4q {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100110111111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b00u32 << 5u32
                    | u32::from(self.Zd) << 2u32
                    | 0b10u32 << 0u32,
            )
        }
    }
}
