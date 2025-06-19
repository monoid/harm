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
        pub const fn new(
            Rs: ::aarchmrs_types::BitValue<5>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rs, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001000001u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
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
        pub const fn new(
            Rs: ::aarchmrs_types::BitValue<5>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rs, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001000001u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
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
        pub const fn new(
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001000011111110u32 << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
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
        pub const fn new(
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001000011111111u32 << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
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
        pub const fn new(
            Rs: ::aarchmrs_types::BitValue<5>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rs, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001000001u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
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
        pub const fn new(
            Rs: ::aarchmrs_types::BitValue<5>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rs, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001000001u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
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
        pub const fn new(
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001000011111110u32 << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
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
        pub const fn new(
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001000011111111u32 << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
