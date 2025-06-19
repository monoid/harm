/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_pn_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilege_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilege_pn_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            vl: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            PNd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                vl: vl.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                PNd: PNd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.vl) << 13u32
                    | 0b00u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.eq) << 3u32
                    | u32::from(self.PNd) << 0u32,
            )
        }
    }
}
pub mod whilehs_pn_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilehs_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilehs_pn_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            vl: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            PNd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                vl: vl.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                PNd: PNd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.vl) << 13u32
                    | 0b01u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.eq) << 3u32
                    | u32::from(self.PNd) << 0u32,
            )
        }
    }
}
pub mod whilegt_pn_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilegt_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilegt_pn_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            vl: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            PNd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                vl: vl.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                PNd: PNd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.vl) << 13u32
                    | 0b00u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.eq) << 3u32
                    | u32::from(self.PNd) << 0u32,
            )
        }
    }
}
pub mod whilehi_pn_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilehi_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilehi_pn_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            vl: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            PNd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                vl: vl.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                PNd: PNd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.vl) << 13u32
                    | 0b01u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.eq) << 3u32
                    | u32::from(self.PNd) << 0u32,
            )
        }
    }
}
pub mod whilelt_pn_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilelt_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilelt_pn_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            vl: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            PNd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                vl: vl.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                PNd: PNd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.vl) << 13u32
                    | 0b00u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.eq) << 3u32
                    | u32::from(self.PNd) << 0u32,
            )
        }
    }
}
pub mod whilelo_pn_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilelo_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilelo_pn_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            vl: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            PNd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                vl: vl.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                PNd: PNd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.vl) << 13u32
                    | 0b01u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.eq) << 3u32
                    | u32::from(self.PNd) << 0u32,
            )
        }
    }
}
pub mod whilele_pn_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilele_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilele_pn_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            vl: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            PNd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                vl: vl.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                PNd: PNd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.vl) << 13u32
                    | 0b00u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.eq) << 3u32
                    | u32::from(self.PNd) << 0u32,
            )
        }
    }
}
pub mod whilels_pn_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilels_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilels_pn_rr_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            vl: impl Into<::aarchmrs_types::BitValue<1>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            eq: impl Into<::aarchmrs_types::BitValue<1>>,
            PNd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rm: Rm.into(),
                vl: vl.into(),
                lt: lt.into(),
                Rn: Rn.into(),
                eq: eq.into(),
                PNd: PNd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.vl) << 13u32
                    | 0b01u32 << 11u32
                    | u32::from(self.lt) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.eq) << 3u32
                    | u32::from(self.PNd) << 0u32,
            )
        }
    }
}
