/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod brkpa_p_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkpa_p_p_pp_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkpa_p_p_pp_ {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            B: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                S: S.into(),
                Pm: Pm.into(),
                Pg: Pg.into(),
                Pn: Pn.into(),
                B: B.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b00u32 << 20u32
                    | u32::from(self.Pm) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | u32::from(self.B) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod brkpas_p_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkpas_p_p_pp_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkpas_p_p_pp_ {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            B: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                S: S.into(),
                Pm: Pm.into(),
                Pg: Pg.into(),
                Pn: Pn.into(),
                B: B.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b00u32 << 20u32
                    | u32::from(self.Pm) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | u32::from(self.B) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod brkpb_p_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkpb_p_p_pp_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkpb_p_p_pp_ {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            B: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                S: S.into(),
                Pm: Pm.into(),
                Pg: Pg.into(),
                Pn: Pn.into(),
                B: B.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b00u32 << 20u32
                    | u32::from(self.Pm) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | u32::from(self.B) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod brkpbs_p_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkpbs_p_p_pp_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkpbs_p_p_pp_ {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            B: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                S: S.into(),
                Pm: Pm.into(),
                Pg: Pg.into(),
                Pn: Pn.into(),
                B: B.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b00u32 << 20u32
                    | u32::from(self.Pm) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | u32::from(self.B) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
