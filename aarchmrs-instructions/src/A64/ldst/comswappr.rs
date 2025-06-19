/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CASP_CP32_comswappr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CASP_CP32_comswappr {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CASP_CP32_comswappr {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod CASPL_CP32_comswappr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CASPL_CP32_comswappr {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CASPL_CP32_comswappr {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod CASPA_CP32_comswappr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CASPA_CP32_comswappr {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CASPA_CP32_comswappr {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod CASPAL_CP32_comswappr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CASPAL_CP32_comswappr {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CASPAL_CP32_comswappr {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod CASP_CP64_comswappr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CASP_CP64_comswappr {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CASP_CP64_comswappr {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod CASPL_CP64_comswappr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CASPL_CP64_comswappr {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CASPL_CP64_comswappr {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod CASPA_CP64_comswappr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CASPA_CP64_comswappr {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CASPA_CP64_comswappr {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod CASPAL_CP64_comswappr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CASPAL_CP64_comswappr {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CASPAL_CP64_comswappr {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
