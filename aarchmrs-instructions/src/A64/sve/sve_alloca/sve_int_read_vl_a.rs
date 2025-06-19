/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod rdvl_r_i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct rdvl_r_i_ {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl rdvl_r_i_ {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000001001011111101010u32 << 11u32
                    | self.imm6.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
