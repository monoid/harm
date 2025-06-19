/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcmge_p_p_z0_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmge_p_p_z0_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmge_p_p_z0_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                lt: lt.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                ne: ne.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01000u32 << 17u32
                    | u32::from(self.lt) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod fcmeq_p_p_z0_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmeq_p_p_z0_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmeq_p_p_z0_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                lt: lt.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01001u32 << 17u32
                    | u32::from(self.lt) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod fcmgt_p_p_z0_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmgt_p_p_z0_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmgt_p_p_z0_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                lt: lt.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                ne: ne.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01000u32 << 17u32
                    | u32::from(self.lt) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod fcmlt_p_p_z0_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmlt_p_p_z0_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmlt_p_p_z0_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                lt: lt.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                ne: ne.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01000u32 << 17u32
                    | u32::from(self.lt) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod fcmne_p_p_z0_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmne_p_p_z0_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmne_p_p_z0_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                lt: lt.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01001u32 << 17u32
                    | u32::from(self.lt) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod fcmle_p_p_z0_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmle_p_p_z0_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmle_p_p_z0_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                lt: lt.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                ne: ne.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01000u32 << 17u32
                    | u32::from(self.lt) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
