/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod rev_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct rev_p_p_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl rev_p_p_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pn: Pn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1101000100000u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
