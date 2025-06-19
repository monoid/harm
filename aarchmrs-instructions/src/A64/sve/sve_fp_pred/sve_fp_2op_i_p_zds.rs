/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fadd_z_p_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fadd_z_p_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fadd_z_p_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                i1: i1.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b011000100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0000u32 << 6u32
                    | u32::from(self.i1) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod fsub_z_p_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fsub_z_p_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fsub_z_p_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                i1: i1.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b011001100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0000u32 << 6u32
                    | u32::from(self.i1) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod fmul_z_p_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmul_z_p_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fmul_z_p_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                i1: i1.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b011010100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0000u32 << 6u32
                    | u32::from(self.i1) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod fsubr_z_p_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fsubr_z_p_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fsubr_z_p_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                i1: i1.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b011011100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0000u32 << 6u32
                    | u32::from(self.i1) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod fmaxnm_z_p_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmaxnm_z_p_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fmaxnm_z_p_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                i1: i1.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b011100100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0000u32 << 6u32
                    | u32::from(self.i1) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod fminnm_z_p_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fminnm_z_p_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fminnm_z_p_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                i1: i1.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b011101100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0000u32 << 6u32
                    | u32::from(self.i1) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod fmax_z_p_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmax_z_p_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fmax_z_p_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                i1: i1.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b011110100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0000u32 << 6u32
                    | u32::from(self.i1) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod fmin_z_p_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmin_z_p_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fmin_z_p_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                i1: i1.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b011111100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0000u32 << 6u32
                    | u32::from(self.i1) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
