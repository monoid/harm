/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod psel_p_ppi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct psel_p_ppi_ {
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl psel_p_ppi_ {
        #[inline]
        pub fn new(
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<3>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                i1: i1.into(),
                tszh: tszh.into(),
                tszl: tszl.into(),
                Rv: Rv.into(),
                Pn: Pn.into(),
                Pm: Pm.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.i1) << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 18u32
                    | u32::from(self.Rv) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.Pn) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
