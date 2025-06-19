/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CSEL_32_condsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CSEL_32_condsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CSEL_32_condsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                o2: o2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011010100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.o2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CSINC_32_condsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CSINC_32_condsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CSINC_32_condsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                o2: o2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011010100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.o2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CSINV_32_condsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CSINV_32_condsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CSINV_32_condsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                o2: o2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011010100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.o2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CSNEG_32_condsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CSNEG_32_condsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CSNEG_32_condsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                o2: o2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011010100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.o2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CSEL_64_condsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CSEL_64_condsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CSEL_64_condsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                o2: o2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011010100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.o2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CSINC_64_condsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CSINC_64_condsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CSINC_64_condsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                o2: o2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011010100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.o2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CSINV_64_condsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CSINV_64_condsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CSINV_64_condsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                o2: o2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.o2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CSNEG_64_condsel {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CSNEG_64_condsel {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CSNEG_64_condsel {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                o2: o2.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.o2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
