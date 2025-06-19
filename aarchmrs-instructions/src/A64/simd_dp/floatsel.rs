/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCSEL_S_floatsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCSEL_S_floatsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCSEL_S_floatsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b11u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCSEL_D_floatsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCSEL_D_floatsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCSEL_D_floatsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b11u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCSEL_H_floatsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCSEL_H_floatsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCSEL_H_floatsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b11u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
