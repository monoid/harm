/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCMP_S_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_S_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_S_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMP_SZ_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_SZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_SZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111000100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMPE_S_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_S_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_S_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMPE_SZ_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_SZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_SZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111000100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMP_D_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_D_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_D_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMP_DZ_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_DZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_DZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111001100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMPE_D_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_D_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_D_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMPE_DZ_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_DZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_DZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111001100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMP_H_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_H_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_H_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMP_HZ_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_HZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_HZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111011100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMPE_H_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_H_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_H_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
pub mod FCMPE_HZ_floatcmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_HZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_HZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111011100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
            )
        }
    }
}
