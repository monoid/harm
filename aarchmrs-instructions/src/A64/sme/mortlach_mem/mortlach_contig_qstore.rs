/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1q_za_p_rrr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1q_za_p_rrr_ {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub ZAt: ::aarchmrs_types::BitValue<4>,
    }
    impl st1q_za_p_rrr_ {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            ZAt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                Rm,
                V,
                Rs,
                Pg,
                Rn,
                ZAt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100001111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.ZAt.into_inner() << 0u32,
            )
        }
    }
}
