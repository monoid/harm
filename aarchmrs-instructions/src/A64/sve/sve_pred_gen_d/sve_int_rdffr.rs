/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod rdffr_p_p_f_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct rdffr_p_p_f_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl rdffr_p_p_f_ {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                S: S.into(),
                Pg: Pg.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b0110001111000u32 << 9u32
                    | u32::from(self.Pg) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod rdffrs_p_p_f_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct rdffrs_p_p_f_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl rdffrs_p_p_f_ {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                S: S.into(),
                Pg: Pg.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b0110001111000u32 << 9u32
                    | u32::from(self.Pg) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
