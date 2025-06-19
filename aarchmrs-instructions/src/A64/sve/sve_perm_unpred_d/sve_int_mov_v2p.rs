/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pmov_p_zi_b {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pmov_p_zi_b {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl pmov_p_zi_b {
        #[inline]
        pub fn new(
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Zn: Zn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0000010100101010001110u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod pmov_p_zi_h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pmov_p_zi_h {
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl pmov_p_zi_h {
        #[inline]
        pub fn new(
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                i1: i1.into(),
                Zn: Zn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101001011u32 << 18u32
                    | u32::from(self.i1) << 17u32
                    | 0b0001110u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod pmov_p_zi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pmov_p_zi_s {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl pmov_p_zi_s {
        #[inline]
        pub fn new(
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                i2: i2.into(),
                Zn: Zn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0000010101101u32 << 19u32
                    | u32::from(self.i2) << 17u32
                    | 0b0001110u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod pmov_p_zi_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pmov_p_zi_d {
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl pmov_p_zi_d {
        #[inline]
        pub fn new(
            i3h: impl Into<::aarchmrs_types::BitValue<1>>,
            i3l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                i3h: i3h.into(),
                i3l: i3l.into(),
                Zn: Zn.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000001011u32 << 23u32
                    | u32::from(self.i3h) << 22u32
                    | 0b101u32 << 19u32
                    | u32::from(self.i3l) << 17u32
                    | 0b0001110u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
