/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshr_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshr_z_mz2_ {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshr_z_mz2_ {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            Zn: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm4, Zn, U, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqrshru_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshru_z_mz2_ {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshru_z_mz2_ {
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
                0b110000011111u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod uqrshr_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqrshr_z_mz2_ {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqrshr_z_mz2_ {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            Zn: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm4, Zn, U, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
