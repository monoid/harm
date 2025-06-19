/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_za_p_rrr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1b_za_p_rrr_ {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub off4: ::aarchmrs_types::BitValue<4>,
    }
    impl st1b_za_p_rrr_ {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            off4: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                V: V.into(),
                Rs: Rs.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                off4: off4.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100000001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.off4) << 0u32,
            )
        }
    }
}
pub mod st1h_za_p_rrr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1h_za_p_rrr_ {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub ZAt: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl st1h_za_p_rrr_ {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAt: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                V: V.into(),
                Rs: Rs.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                ZAt: ZAt.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100000011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.ZAt) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod st1w_za_p_rrr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1w_za_p_rrr_ {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub ZAt: ::aarchmrs_types::BitValue<2>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl st1w_za_p_rrr_ {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAt: impl Into<::aarchmrs_types::BitValue<2>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                V: V.into(),
                Rs: Rs.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                ZAt: ZAt.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100000101u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.ZAt) << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod st1d_za_p_rrr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1d_za_p_rrr_ {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub ZAt: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl st1d_za_p_rrr_ {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAt: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                V: V.into(),
                Rs: Rs.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                ZAt: ZAt.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100000111u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.ZAt) << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
