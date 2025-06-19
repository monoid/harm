/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STLLRB_SL32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLLRB_SL32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLLRB_SL32_ldstord {
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
                0b0000100010011111011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLRB_SL32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLRB_SL32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLRB_SL32_ldstord {
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
                0b0000100010011111111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDLARB_LR32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDLARB_LR32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDLARB_LR32_ldstord {
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
                0b0000100011011111011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDARB_LR32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDARB_LR32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDARB_LR32_ldstord {
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
                0b0000100011011111111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLLRH_SL32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLLRH_SL32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLLRH_SL32_ldstord {
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
                0b0100100010011111011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLRH_SL32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLRH_SL32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLRH_SL32_ldstord {
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
                0b0100100010011111111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDLARH_LR32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDLARH_LR32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDLARH_LR32_ldstord {
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
                0b0100100011011111011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDARH_LR32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDARH_LR32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDARH_LR32_ldstord {
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
                0b0100100011011111111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLLR_SL32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLLR_SL32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLLR_SL32_ldstord {
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
                0b1000100010011111011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLR_SL32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLR_SL32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLR_SL32_ldstord {
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
                0b1000100010011111111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDLAR_LR32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDLAR_LR32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDLAR_LR32_ldstord {
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
                0b1000100011011111011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAR_LR32_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAR_LR32_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAR_LR32_ldstord {
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
                0b1000100011011111111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLLR_SL64_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLLR_SL64_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLLR_SL64_ldstord {
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
                0b1100100010011111011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLR_SL64_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLR_SL64_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLR_SL64_ldstord {
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
                0b1100100010011111111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDLAR_LR64_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDLAR_LR64_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDLAR_LR64_ldstord {
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
                0b1100100011011111011111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAR_LR64_ldstord {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAR_LR64_ldstord {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAR_LR64_ldstord {
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
                0b1100100011011111111111u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
