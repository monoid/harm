/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshrn_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshrn_z_mz2_ {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshrn_z_mz2_ {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm4, U, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001011011u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.U.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqrshrun_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshrun_z_mz2_ {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshrun_z_mz2_ {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            Zn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm4, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001011011u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b000010u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod uqrshrn_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqrshrn_z_mz2_ {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqrshrn_z_mz2_ {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm4, U, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001011011u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.U.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
