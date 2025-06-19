/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ext_z_zi_con {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ext_z_zi_con {
        pub imm8h: ::aarchmrs_types::BitValue<5>,
        pub imm8l: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl ext_z_zi_con {
        #[inline]
        pub const fn new(
            imm8h: ::aarchmrs_types::BitValue<5>,
            imm8l: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                imm8h,
                imm8l,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101011u32 << 21u32
                    | self.imm8h.into_inner() << 16u32
                    | 0b000u32 << 13u32
                    | self.imm8l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
