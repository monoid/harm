/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sel_mz_p_zz_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sel_mz_p_zz_4 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl sel_mz_p_zz_4 {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zd: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                PNg: PNg.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Zm) << 18u32
                    | 0b01100u32 << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b00u32 << 5u32
                    | u32::from(self.Zd) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
