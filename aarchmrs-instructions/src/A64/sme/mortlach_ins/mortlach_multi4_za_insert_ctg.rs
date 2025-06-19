/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_za_mz4_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_za_mz4_1 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_za_mz4_1 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                Zn: Zn.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000001000u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b011u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b0000u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
