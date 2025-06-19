/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sel_mz_p_zz_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sel_mz_p_zz_2 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
    }
    impl sel_mz_p_zz_2 {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<4>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                Zm,
                PNg,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100u32 << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.Zd.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
