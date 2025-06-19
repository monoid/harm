/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dup_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct dup_z_zi_ {
        pub imm2: ::aarchmrs_types::BitValue<2>,
        pub tsz: ::aarchmrs_types::BitValue<5>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl dup_z_zi_ {
        #[inline]
        pub const fn new(
            imm2: ::aarchmrs_types::BitValue<2>,
            tsz: ::aarchmrs_types::BitValue<5>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm2, tsz, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.imm2.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tsz.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
