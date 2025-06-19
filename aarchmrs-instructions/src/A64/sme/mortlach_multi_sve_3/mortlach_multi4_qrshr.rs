/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshr_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshr_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshr_z_mz4_ {
        #[inline]
        pub fn new(
            tsize: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tsize: tsize.into(),
                imm5: imm5.into(),
                N: N.into(),
                Zn: Zn.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.tsize) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b11011u32 << 11u32
                    | u32::from(self.N) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b0u32 << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqrshru_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshru_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshru_z_mz4_ {
        #[inline]
        pub fn new(
            tsize: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tsize: tsize.into(),
                imm5: imm5.into(),
                N: N.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.tsize) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b11011u32 << 11u32
                    | u32::from(self.N) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b10u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqrshrn_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshrn_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshrn_z_mz4_ {
        #[inline]
        pub fn new(
            tsize: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tsize: tsize.into(),
                imm5: imm5.into(),
                N: N.into(),
                Zn: Zn.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.tsize) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b11011u32 << 11u32
                    | u32::from(self.N) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b0u32 << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqrshrun_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshrun_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshrun_z_mz4_ {
        #[inline]
        pub fn new(
            tsize: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tsize: tsize.into(),
                imm5: imm5.into(),
                N: N.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.tsize) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b11011u32 << 11u32
                    | u32::from(self.N) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b10u32 << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqrshr_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqrshr_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqrshr_z_mz4_ {
        #[inline]
        pub fn new(
            tsize: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tsize: tsize.into(),
                imm5: imm5.into(),
                N: N.into(),
                Zn: Zn.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.tsize) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b11011u32 << 11u32
                    | u32::from(self.N) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b0u32 << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqrshrn_z_mz4_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqrshrn_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqrshrn_z_mz4_ {
        #[inline]
        pub fn new(
            tsize: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tsize: tsize.into(),
                imm5: imm5.into(),
                N: N.into(),
                Zn: Zn.into(),
                U: U.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.tsize) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b11011u32 << 11u32
                    | u32::from(self.N) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b0u32 << 6u32
                    | u32::from(self.U) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
