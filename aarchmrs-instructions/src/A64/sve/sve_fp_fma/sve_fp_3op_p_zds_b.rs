/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmad_z_p_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmad_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fmad_z_p_zzz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Za: impl Into<::aarchmrs_types::BitValue<5>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Za: Za.into(),
                N: N.into(),
                op: op.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Za) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.N) << 14u32
                    | u32::from(self.op) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod fmsb_z_p_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmsb_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fmsb_z_p_zzz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Za: impl Into<::aarchmrs_types::BitValue<5>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Za: Za.into(),
                N: N.into(),
                op: op.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Za) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.N) << 14u32
                    | u32::from(self.op) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod fnmad_z_p_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fnmad_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fnmad_z_p_zzz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Za: impl Into<::aarchmrs_types::BitValue<5>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Za: Za.into(),
                N: N.into(),
                op: op.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Za) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.N) << 14u32
                    | u32::from(self.op) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod fnmsb_z_p_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fnmsb_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fnmsb_z_p_zzz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Za: impl Into<::aarchmrs_types::BitValue<5>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Za: Za.into(),
                N: N.into(),
                op: op.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Za) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.N) << 14u32
                    | u32::from(self.op) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
