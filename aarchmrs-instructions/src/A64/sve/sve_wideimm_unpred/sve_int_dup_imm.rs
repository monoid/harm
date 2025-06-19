/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dup_z_i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct dup_z_i_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl dup_z_i_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            sh: ::aarchmrs_types::BitValue<1>,
            imm8: ::aarchmrs_types::BitValue<8>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, sh, imm8, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11100011u32 << 14u32
                    | self.sh.into_inner() << 13u32
                    | self.imm8.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
