/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqcvt_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqcvt_z_mz4_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqcvt_z_mz4_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zn: Zn.into(),
                N: N.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.sz) << 23u32
                    | 0b0110011111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | u32::from(self.N) << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqcvtu_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqcvtu_z_mz4_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqcvtu_z_mz4_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zn: Zn.into(),
                N: N.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.sz) << 23u32
                    | 0b1110011111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | u32::from(self.N) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqcvtn_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqcvtn_z_mz4_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqcvtn_z_mz4_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zn: Zn.into(),
                N: N.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.sz) << 23u32
                    | 0b0110011111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | u32::from(self.N) << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqcvtun_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqcvtun_z_mz4_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqcvtun_z_mz4_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zn: Zn.into(),
                N: N.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.sz) << 23u32
                    | 0b1110011111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | u32::from(self.N) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqcvt_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqcvt_z_mz4_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqcvt_z_mz4_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zn: Zn.into(),
                N: N.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.sz) << 23u32
                    | 0b0110011111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | u32::from(self.N) << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqcvtn_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqcvtn_z_mz4_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqcvtn_z_mz4_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zn: Zn.into(),
                N: N.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.sz) << 23u32
                    | 0b0110011111000u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | u32::from(self.N) << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
