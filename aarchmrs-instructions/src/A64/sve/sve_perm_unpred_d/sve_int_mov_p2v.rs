/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pmov_z_pi_b {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pmov_z_pi_b {
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl pmov_z_pi_b {
        #[inline]
        pub const fn new(
            Pn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Pn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101001010110011100u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod pmov_z_pi_h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pmov_z_pi_h {
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl pmov_z_pi_h {
        #[inline]
        pub const fn new(
            i1: ::aarchmrs_types::BitValue<1>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { i1, Pn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101001011u32 << 18u32
                    | self.i1.into_inner() << 17u32
                    | 0b10011100u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod pmov_z_pi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pmov_z_pi_s {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl pmov_z_pi_s {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { i2, Pn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0000010101101u32 << 19u32
                    | self.i2.into_inner() << 17u32
                    | 0b10011100u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod pmov_z_pi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pmov_z_pi_d {
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl pmov_z_pi_d {
        #[inline]
        pub const fn new(
            i3h: ::aarchmrs_types::BitValue<1>,
            i3l: ::aarchmrs_types::BitValue<2>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { i3h, i3l, Pn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000001011u32 << 23u32
                    | self.i3h.into_inner() << 22u32
                    | 0b101u32 << 19u32
                    | self.i3l.into_inner() << 17u32
                    | 0b10011100u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
