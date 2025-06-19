/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod brka_p_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brka_p_p_p_ {
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brka_p_p_p_ {
        #[inline]
        pub fn new(
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Pg: Pg.into(),
                Pn: Pn.into(),
                M: M.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010001000001u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | u32::from(self.M) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod brkas_p_p_p_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkas_p_p_p_z {
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkas_p_p_p_z {
        #[inline]
        pub fn new(
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Pg: Pg.into(),
                Pn: Pn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010101000001u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod brkb_p_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkb_p_p_p_ {
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkb_p_p_p_ {
        #[inline]
        pub fn new(
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Pg: Pg.into(),
                Pn: Pn.into(),
                M: M.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011001000001u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | u32::from(self.M) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod brkbs_p_p_p_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkbs_p_p_p_z {
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkbs_p_p_p_z {
        #[inline]
        pub fn new(
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Pg: Pg.into(),
                Pn: Pn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011101000001u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
