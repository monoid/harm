/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcmge_p_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmge_p_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub cmph: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub cmpl: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmge_p_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            cmph: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            cmpl: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                cmph: cmph.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                cmpl: cmpl.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.cmph) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.cmpl) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod fcmuo_p_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmuo_p_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmuo_p_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
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
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod facge_p_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct facge_p_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl facge_p_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
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
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod facgt_p_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct facgt_p_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl facgt_p_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
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
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod fcmgt_p_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmgt_p_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub cmph: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub cmpl: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmgt_p_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            cmph: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            cmpl: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                cmph: cmph.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                cmpl: cmpl.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.cmph) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.cmpl) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod fcmeq_p_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmeq_p_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub cmph: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub cmpl: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmeq_p_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            cmph: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            cmpl: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                cmph: cmph.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                cmpl: cmpl.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.cmph) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.cmpl) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod fcmne_p_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcmne_p_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub cmph: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub cmpl: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl fcmne_p_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            cmph: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            cmpl: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                cmph: cmph.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                cmpl: cmpl.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.cmph) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.cmpl) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
