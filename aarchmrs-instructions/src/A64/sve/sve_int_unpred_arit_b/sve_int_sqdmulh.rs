/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqdmulh_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdmulh_z_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdmulh_z_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqrdmulh_z_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrdmulh_z_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrdmulh_z_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b011101u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
