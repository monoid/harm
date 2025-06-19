/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_p_p_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilege_p_p_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub sf: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl whilege_p_p_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            sf: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                sf: sf.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.sf) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.eq) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod whilehs_p_p_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilehs_p_p_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub sf: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl whilehs_p_p_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            sf: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                sf: sf.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.sf) << 12u32
                    | 0b1u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.eq) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod whilegt_p_p_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilegt_p_p_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub sf: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl whilegt_p_p_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            sf: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                sf: sf.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.sf) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.eq) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod whilehi_p_p_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilehi_p_p_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub sf: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl whilehi_p_p_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            sf: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                sf: sf.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.sf) << 12u32
                    | 0b1u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.eq) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod whilelt_p_p_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilelt_p_p_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub sf: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl whilelt_p_p_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            sf: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                sf: sf.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.sf) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.eq) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod whilelo_p_p_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilelo_p_p_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub sf: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl whilelo_p_p_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            sf: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                sf: sf.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.sf) << 12u32
                    | 0b1u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.eq) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod whilele_p_p_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilele_p_p_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub sf: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl whilele_p_p_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            sf: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                sf: sf.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.sf) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.eq) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod whilels_p_p_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilels_p_p_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub sf: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl whilels_p_p_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            sf: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                sf: sf.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.sf) << 12u32
                    | 0b1u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.eq) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
