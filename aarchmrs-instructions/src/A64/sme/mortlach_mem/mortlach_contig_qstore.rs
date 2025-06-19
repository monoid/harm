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
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAt: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                V: V.into(),
                Rs: Rs.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                ZAt: ZAt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100001111u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.ZAt) << 0u32,
            )
        }
    }
}
