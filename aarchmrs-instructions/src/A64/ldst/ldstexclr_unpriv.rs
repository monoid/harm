/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STTXR_SR32_ldstexclr_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTXR_SR32_ldstexclr_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTXR_SR32_ldstexclr_unpriv {
        #[inline]
        pub const fn new(
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rs, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLTXR_SR32_ldstexclr_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLTXR_SR32_ldstexclr_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLTXR_SR32_ldstexclr_unpriv {
        #[inline]
        pub const fn new(
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rs, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDTXR_LR32_ldstexclr_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTXR_LR32_ldstexclr_unpriv {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTXR_LR32_ldstexclr_unpriv {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000100101011111011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDATXR_LR32_ldstexclr_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDATXR_LR32_ldstexclr_unpriv {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDATXR_LR32_ldstexclr_unpriv {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000100101011111111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STTXR_SR64_ldstexclr_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTXR_SR64_ldstexclr_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTXR_SR64_ldstexclr_unpriv {
        #[inline]
        pub const fn new(
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rs, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLTXR_SR64_ldstexclr_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLTXR_SR64_ldstexclr_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLTXR_SR64_ldstexclr_unpriv {
        #[inline]
        pub const fn new(
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rs, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDTXR_LR64_ldstexclr_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTXR_LR64_ldstexclr_unpriv {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTXR_LR64_ldstexclr_unpriv {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100100101011111011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDATXR_LR64_ldstexclr_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDATXR_LR64_ldstexclr_unpriv {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDATXR_LR64_ldstexclr_unpriv {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100100101011111111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
