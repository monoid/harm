/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STXP_SP32_ldstexclp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STXP_SP32_ldstexclp {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STXP_SP32_ldstexclp {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STLXP_SP32_ldstexclp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLXP_SP32_ldstexclp {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLXP_SP32_ldstexclp {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDXP_LP32_ldstexclp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDXP_LP32_ldstexclp {
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDXP_LP32_ldstexclp {
        #[inline]
        pub fn new(
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001000011111110u32 << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDAXP_LP32_ldstexclp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAXP_LP32_ldstexclp {
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAXP_LP32_ldstexclp {
        #[inline]
        pub fn new(
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001000011111111u32 << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STXP_SP64_ldstexclp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STXP_SP64_ldstexclp {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STXP_SP64_ldstexclp {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STLXP_SP64_ldstexclp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLXP_SP64_ldstexclp {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLXP_SP64_ldstexclp {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDXP_LP64_ldstexclp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDXP_LP64_ldstexclp {
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDXP_LP64_ldstexclp {
        #[inline]
        pub fn new(
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001000011111110u32 << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDAXP_LP64_ldstexclp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAXP_LP64_ldstexclp {
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAXP_LP64_ldstexclp {
        #[inline]
        pub fn new(
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001000011111111u32 << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
