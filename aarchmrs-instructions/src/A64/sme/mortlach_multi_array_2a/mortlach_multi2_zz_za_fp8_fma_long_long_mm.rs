/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlall_za32_z8z8w_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlall_za32_z8z8w_2x2 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl fmlall_za32_z8z8w_2x2 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001101u32 << 21u32
                    | u32::from(self.Zm) << 17u32
                    | 0b00u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b10000u32 << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
