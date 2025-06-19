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
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                U,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b0u32 << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
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
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b10u32 << 5u32
                    | self.Zd.into_inner() << 0u32,
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
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                U,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b0u32 << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
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
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b10u32 << 5u32
                    | self.Zd.into_inner() << 0u32,
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
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                U,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b0u32 << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
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
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                U,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b0u32 << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
