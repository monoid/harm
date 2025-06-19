/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlall_za32_z8z8v_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlall_za32_z8z8v_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl fmlall_za32_z8z8v_1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010011u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b001u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b000u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
