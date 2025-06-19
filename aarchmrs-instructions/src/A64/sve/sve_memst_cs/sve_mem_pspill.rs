/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod str_p_bi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct str_p_bi_ {
        pub imm9h: ::aarchmrs_types::BitValue<6>,
        pub imm9l: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pt: ::aarchmrs_types::BitValue<4>,
    }
    impl str_p_bi_ {
        #[inline]
        pub const fn new(
            imm9h: ::aarchmrs_types::BitValue<6>,
            imm9l: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm9h,
                imm9l,
                Rn,
                Pt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010110u32 << 22u32
                    | self.imm9h.into_inner() << 16u32
                    | 0b000u32 << 13u32
                    | self.imm9l.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pt.into_inner() << 0u32,
            )
        }
    }
}
