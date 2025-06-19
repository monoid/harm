/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod brka_p_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brka_p_p_p_ {
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brka_p_p_p_ {
        #[inline]
        pub const fn new(
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            M: ::aarchmrs_types::BitValue<1>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Pg, Pn, M, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010001000001u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.M.into_inner() << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod brkas_p_p_p_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkas_p_p_p_z {
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkas_p_p_p_z {
        #[inline]
        pub const fn new(
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010101000001u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod brkb_p_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkb_p_p_p_ {
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkb_p_p_p_ {
        #[inline]
        pub const fn new(
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            M: ::aarchmrs_types::BitValue<1>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Pg, Pn, M, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011001000001u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.M.into_inner() << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod brkbs_p_p_p_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkbs_p_p_p_z {
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkbs_p_p_p_z {
        #[inline]
        pub const fn new(
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011101000001u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
