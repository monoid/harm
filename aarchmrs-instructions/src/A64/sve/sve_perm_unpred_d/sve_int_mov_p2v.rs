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
        pub fn new(
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Pn: Pn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101001010110011100u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | u32::from(self.Zd) << 0u32,
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
        pub fn new(
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i1: i1.into(),
                Pn: Pn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101001011u32 << 18u32
                    | u32::from(self.i1) << 17u32
                    | 0b10011100u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | u32::from(self.Zd) << 0u32,
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
        pub fn new(
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i2: i2.into(),
                Pn: Pn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0000010101101u32 << 19u32
                    | u32::from(self.i2) << 17u32
                    | 0b10011100u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | u32::from(self.Zd) << 0u32,
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
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<2>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                i3l: i3l.into(),
                Pn: Pn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000001011u32 << 23u32
                    | u32::from(self.i3h) << 22u32
                    | 0b101u32 << 19u32
                    | u32::from(self.i3l) << 17u32
                    | 0b10011100u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
