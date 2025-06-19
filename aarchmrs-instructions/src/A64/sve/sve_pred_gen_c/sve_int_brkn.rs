/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod brkn_p_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkn_p_p_pp_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pdm: ::aarchmrs_types::BitValue<4>,
    }
    impl brkn_p_p_pp_ {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Pdm: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                S: S.into(),
                Pg: Pg.into(),
                Pn: Pn.into(),
                Pdm: Pdm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b01100001u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pdm) << 0u32,
            )
        }
    }
}
pub mod brkns_p_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkns_p_p_pp_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pdm: ::aarchmrs_types::BitValue<4>,
    }
    impl brkns_p_p_pp_ {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Pdm: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                S: S.into(),
                Pg: Pg.into(),
                Pn: Pn.into(),
                Pdm: Pdm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b01100001u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pdm) << 0u32,
            )
        }
    }
}
