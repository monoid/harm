/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod str_z_bi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct str_z_bi_ {
        pub imm9h: ::aarchmrs_types::BitValue<6>,
        pub imm9l: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl str_z_bi_ {
        #[inline]
        pub const fn new(
            imm9h: ::aarchmrs_types::BitValue<6>,
            imm9l: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                imm9h,
                imm9l,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010110u32 << 22u32
                    | self.imm9h.into_inner() << 16u32
                    | 0b010u32 << 13u32
                    | self.imm9l.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
