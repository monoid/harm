/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_mz_za4_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_mz_za4_1 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub off3: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_mz_za4_1 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                off3: off3.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000001100u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b01100u32 << 8u32
                    | u32::from(self.off3) << 5u32
                    | u32::from(self.Zd) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
