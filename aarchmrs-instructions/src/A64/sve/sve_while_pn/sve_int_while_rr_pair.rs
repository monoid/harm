/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilege_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilege_pp_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<3>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                Pd: Pd.into(),
                eq: eq.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01010u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.Pd) << 1u32
                    | u32::from(self.eq) << 0u32,
            )
        }
    }
}
pub mod whilehs_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilehs_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilehs_pp_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<3>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                Pd: Pd.into(),
                eq: eq.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01011u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.Pd) << 1u32
                    | u32::from(self.eq) << 0u32,
            )
        }
    }
}
pub mod whilegt_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilegt_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilegt_pp_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<3>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                Pd: Pd.into(),
                eq: eq.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01010u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.Pd) << 1u32
                    | u32::from(self.eq) << 0u32,
            )
        }
    }
}
pub mod whilehi_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilehi_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilehi_pp_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<3>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                Pd: Pd.into(),
                eq: eq.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01011u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.Pd) << 1u32
                    | u32::from(self.eq) << 0u32,
            )
        }
    }
}
pub mod whilelt_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilelt_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilelt_pp_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<3>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                Pd: Pd.into(),
                eq: eq.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01010u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.Pd) << 1u32
                    | u32::from(self.eq) << 0u32,
            )
        }
    }
}
pub mod whilelo_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilelo_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilelo_pp_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<3>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                Pd: Pd.into(),
                eq: eq.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01011u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.Pd) << 1u32
                    | u32::from(self.eq) << 0u32,
            )
        }
    }
}
pub mod whilele_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilele_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilele_pp_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<3>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                Pd: Pd.into(),
                eq: eq.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01010u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.Pd) << 1u32
                    | u32::from(self.eq) << 0u32,
            )
        }
    }
}
pub mod whilels_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilels_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilels_pp_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<3>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                Pd: Pd.into(),
                eq: eq.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01011u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.Pd) << 1u32
                    | u32::from(self.eq) << 0u32,
            )
        }
    }
}
