/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cntp_r_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cntp_r_p_p_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl cntp_r_p_p_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Pn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10000010u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod firstp_r_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct firstp_r_p_p_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl firstp_r_p_p_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Pn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10000110u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod lastp_r_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lastp_r_p_p_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl lastp_r_p_p_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Pn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10001010u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
