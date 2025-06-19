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
        pub const fn new(
            i1: ::aarchmrs_types::BitValue<1>,
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<3>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                i1,
                tszh,
                tszl,
                Rv,
                Pn,
                Pm,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.i1.into_inner() << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 18u32
                    | self.Rv.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pn.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pm.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
