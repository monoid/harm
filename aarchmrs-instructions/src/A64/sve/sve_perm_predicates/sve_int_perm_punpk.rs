/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod punpklo_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct punpklo_p_p_ {
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl punpklo_p_p_ {
        #[inline]
        pub fn new(
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                H: H.into(),
                Pn: Pn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000001010011000u32 << 17u32
                    | u32::from(self.H) << 16u32
                    | 0b0100000u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod punpkhi_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct punpkhi_p_p_ {
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl punpkhi_p_p_ {
        #[inline]
        pub fn new(
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                H: H.into(),
                Pn: Pn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000001010011000u32 << 17u32
                    | u32::from(self.H) << 16u32
                    | 0b0100000u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
