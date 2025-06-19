/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDADDB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDB_32_memop {
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
                0b00111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRB_32_memop {
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
                0b00111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORB_32_memop {
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
                0b00111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETB_32_memop {
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
                0b00111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXB_32_memop {
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
                0b00111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINB_32_memop {
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
                0b00111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXB_32_memop {
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
                0b00111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINB_32_memop {
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
                0b00111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPB_32_memop {
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
                0b00111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWCLR_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWCLR_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWCLR_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSWP_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSWP_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSWP_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSET_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSET_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSET_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDLB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDLB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDLB_32_memop {
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
                0b00111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRLB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRLB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRLB_32_memop {
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
                0b00111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORLB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORLB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORLB_32_memop {
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
                0b00111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETLB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETLB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETLB_32_memop {
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
                0b00111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXLB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXLB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXLB_32_memop {
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
                0b00111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINLB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINLB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINLB_32_memop {
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
                0b00111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXLB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXLB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXLB_32_memop {
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
                0b00111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINLB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINLB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINLB_32_memop {
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
                0b00111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPLB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPLB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPLB_32_memop {
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
                0b00111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWCLRL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWCLRL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWCLRL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSWPL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSWPL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSWPL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSETL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSETL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSETL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDAB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDAB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDAB_32_memop {
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
                0b00111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRAB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRAB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRAB_32_memop {
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
                0b00111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORAB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORAB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORAB_32_memop {
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
                0b00111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETAB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETAB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETAB_32_memop {
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
                0b00111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXAB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXAB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXAB_32_memop {
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
                0b00111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINAB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINAB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINAB_32_memop {
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
                0b00111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXAB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXAB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXAB_32_memop {
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
                0b00111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINAB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINAB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINAB_32_memop {
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
                0b00111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPAB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPAB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPAB_32_memop {
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
                0b00111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWCLRA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWCLRA_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWCLRA_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSWPA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSWPA_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSWPA_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSETA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSETA_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSETA_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDAPRB_32L_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPRB_32L_memop {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPRB_32L_memop {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011100010111111110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDALB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDALB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDALB_32_memop {
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
                0b00111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRALB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRALB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRALB_32_memop {
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
                0b00111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORALB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORALB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORALB_32_memop {
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
                0b00111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETALB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETALB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETALB_32_memop {
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
                0b00111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXALB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXALB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXALB_32_memop {
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
                0b00111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINALB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINALB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINALB_32_memop {
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
                0b00111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXALB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXALB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXALB_32_memop {
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
                0b00111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINALB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINALB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINALB_32_memop {
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
                0b00111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPALB_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPALB_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPALB_32_memop {
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
                0b00111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWCLRAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWCLRAL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWCLRAL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSWPAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSWPAL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSWPAL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSETAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSETAL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSETAL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDH_32_memop {
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
                0b01111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRH_32_memop {
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
                0b01111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORH_32_memop {
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
                0b01111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETH_32_memop {
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
                0b01111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXH_32_memop {
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
                0b01111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINH_32_memop {
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
                0b01111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXH_32_memop {
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
                0b01111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINH_32_memop {
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
                0b01111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPH_32_memop {
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
                0b01111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSCLR_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSCLR_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSCLR_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSSWP_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSSWP_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSSWP_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSSET_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSSET_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSSET_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDLH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDLH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDLH_32_memop {
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
                0b01111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRLH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRLH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRLH_32_memop {
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
                0b01111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORLH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORLH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORLH_32_memop {
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
                0b01111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETLH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETLH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETLH_32_memop {
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
                0b01111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXLH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXLH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXLH_32_memop {
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
                0b01111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINLH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINLH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINLH_32_memop {
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
                0b01111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXLH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXLH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXLH_32_memop {
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
                0b01111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINLH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINLH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINLH_32_memop {
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
                0b01111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPLH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPLH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPLH_32_memop {
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
                0b01111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSCLRL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSCLRL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSCLRL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSSWPL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSSWPL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSSWPL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSSETL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSSETL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSSETL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDAH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDAH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDAH_32_memop {
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
                0b01111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRAH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRAH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRAH_32_memop {
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
                0b01111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORAH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORAH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORAH_32_memop {
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
                0b01111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETAH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETAH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETAH_32_memop {
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
                0b01111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXAH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXAH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXAH_32_memop {
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
                0b01111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINAH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINAH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINAH_32_memop {
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
                0b01111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXAH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXAH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXAH_32_memop {
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
                0b01111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINAH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINAH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINAH_32_memop {
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
                0b01111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPAH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPAH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPAH_32_memop {
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
                0b01111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSCLRA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSCLRA_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSCLRA_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSSWPA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSSWPA_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSSWPA_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSSETA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSSETA_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSSETA_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDAPRH_32L_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPRH_32L_memop {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPRH_32L_memop {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111100010111111110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDALH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDALH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDALH_32_memop {
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
                0b01111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRALH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRALH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRALH_32_memop {
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
                0b01111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORALH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORALH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORALH_32_memop {
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
                0b01111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETALH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETALH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETALH_32_memop {
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
                0b01111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXALH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXALH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXALH_32_memop {
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
                0b01111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINALH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINALH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINALH_32_memop {
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
                0b01111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXALH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXALH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXALH_32_memop {
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
                0b01111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINALH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINALH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINALH_32_memop {
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
                0b01111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPALH_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPALH_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPALH_32_memop {
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
                0b01111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSCLRAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSCLRAL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSCLRAL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSSWPAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSSWPAL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSSWPAL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RCWSSETAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RCWSSETAL_64_memop {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RCWSSETAL_64_memop {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                Rs: Rs.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.S) << 30u32
                    | 0b111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADD_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADD_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADD_32_memop {
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
                0b10111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLR_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLR_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLR_32_memop {
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
                0b10111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEOR_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEOR_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEOR_32_memop {
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
                0b10111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSET_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSET_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSET_32_memop {
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
                0b10111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAX_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAX_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAX_32_memop {
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
                0b10111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMIN_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMIN_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMIN_32_memop {
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
                0b10111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAX_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAX_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAX_32_memop {
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
                0b10111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMIN_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMIN_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMIN_32_memop {
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
                0b10111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWP_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWP_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWP_32_memop {
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
                0b10111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDL_32_memop {
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
                0b10111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRL_32_memop {
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
                0b10111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORL_32_memop {
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
                0b10111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETL_32_memop {
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
                0b10111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXL_32_memop {
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
                0b10111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINL_32_memop {
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
                0b10111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXL_32_memop {
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
                0b10111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINL_32_memop {
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
                0b10111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPL_32_memop {
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
                0b10111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDA_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDA_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDA_32_memop {
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
                0b10111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRA_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRA_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRA_32_memop {
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
                0b10111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORA_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORA_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORA_32_memop {
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
                0b10111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETA_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETA_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETA_32_memop {
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
                0b10111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXA_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXA_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXA_32_memop {
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
                0b10111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINA_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINA_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINA_32_memop {
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
                0b10111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXA_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXA_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXA_32_memop {
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
                0b10111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINA_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINA_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINA_32_memop {
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
                0b10111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPA_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPA_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPA_32_memop {
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
                0b10111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDAPR_32L_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPR_32L_memop {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPR_32L_memop {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1011100010111111110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDAL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDAL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDAL_32_memop {
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
                0b10111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRAL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRAL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRAL_32_memop {
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
                0b10111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORAL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORAL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORAL_32_memop {
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
                0b10111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETAL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETAL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETAL_32_memop {
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
                0b10111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXAL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXAL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXAL_32_memop {
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
                0b10111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINAL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINAL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINAL_32_memop {
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
                0b10111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXAL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXAL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXAL_32_memop {
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
                0b10111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINAL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINAL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINAL_32_memop {
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
                0b10111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPAL_32_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPAL_32_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPAL_32_memop {
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
                0b10111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADD_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADD_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADD_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLR_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLR_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLR_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEOR_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEOR_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEOR_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSET_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSET_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSET_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAX_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAX_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAX_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMIN_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMIN_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMIN_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAX_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAX_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAX_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMIN_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMIN_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMIN_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWP_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWP_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWP_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod ST64BV0_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST64BV0_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST64BV0_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod ST64BV_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST64BV_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST64BV_64_memop {
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
                0b11111000001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b101100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod ST64B_64L_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST64B_64L_memop {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST64B_64L_memop {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1111100000111111100100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LD64B_64L_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD64B_64L_memop {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD64B_64L_memop {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1111100000111111110100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDL_64_memop {
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
                0b11111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRL_64_memop {
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
                0b11111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORL_64_memop {
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
                0b11111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETL_64_memop {
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
                0b11111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXL_64_memop {
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
                0b11111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINL_64_memop {
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
                0b11111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXL_64_memop {
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
                0b11111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINL_64_memop {
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
                0b11111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPL_64_memop {
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
                0b11111000011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDA_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDA_64_memop {
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
                0b11111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRA_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRA_64_memop {
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
                0b11111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORA_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORA_64_memop {
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
                0b11111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETA_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETA_64_memop {
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
                0b11111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXA_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXA_64_memop {
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
                0b11111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINA_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINA_64_memop {
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
                0b11111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXA_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXA_64_memop {
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
                0b11111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINA_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINA_64_memop {
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
                0b11111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPA_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPA_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPA_64_memop {
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
                0b11111000101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDAPR_64L_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPR_64L_memop {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPR_64L_memop {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1111100010111111110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDADDAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDADDAL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDADDAL_64_memop {
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
                0b11111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDCLRAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDCLRAL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDCLRAL_64_memop {
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
                0b11111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDEORAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDEORAL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDEORAL_64_memop {
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
                0b11111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSETAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSETAL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSETAL_64_memop {
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
                0b11111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMAXAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMAXAL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMAXAL_64_memop {
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
                0b11111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDSMINAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDSMINAL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDSMINAL_64_memop {
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
                0b11111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMAXAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMAXAL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMAXAL_64_memop {
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
                0b11111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDUMINAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUMINAL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUMINAL_64_memop {
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
                0b11111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPAL_64_memop {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPAL_64_memop {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPAL_64_memop {
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
                0b11111000111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFADD_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFADD_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFADD_16 {
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
                0b00111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMAX_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMAX_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMAX_16 {
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
                0b00111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMIN_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMIN_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMIN_16 {
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
                0b00111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMAXNM_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMAXNM_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMAXNM_16 {
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
                0b00111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMINNM_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMINNM_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMINNM_16 {
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
                0b00111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STBFADD_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STBFADD_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STBFADD_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STBFMAX_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STBFMAX_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STBFMAX_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STBFMIN_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STBFMIN_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STBFMIN_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STBFMAXNM_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STBFMAXNM_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STBFMAXNM_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STBFMINNM_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STBFMINNM_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STBFMINNM_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STBFADDL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STBFADDL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STBFADDL_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STBFMAXL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STBFMAXL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STBFMAXL_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STBFMINL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STBFMINL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STBFMINL_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STBFMAXNML_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STBFMAXNML_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STBFMAXNML_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STBFMINNML_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STBFMINNML_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STBFMINNML_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod LDBFADDL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFADDL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFADDL_16 {
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
                0b00111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMAXL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMAXL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMAXL_16 {
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
                0b00111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMINL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMINL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMINL_16 {
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
                0b00111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMAXNML_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMAXNML_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMAXNML_16 {
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
                0b00111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMINNML_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMINNML_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMINNML_16 {
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
                0b00111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFADDA_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFADDA_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFADDA_16 {
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
                0b00111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMAXA_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMAXA_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMAXA_16 {
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
                0b00111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMINA_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMINA_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMINA_16 {
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
                0b00111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMAXNMA_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMAXNMA_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMAXNMA_16 {
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
                0b00111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMINNMA_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMINNMA_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMINNMA_16 {
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
                0b00111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFADDAL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFADDAL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFADDAL_16 {
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
                0b00111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMAXAL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMAXAL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMAXAL_16 {
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
                0b00111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMINAL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMINAL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMINAL_16 {
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
                0b00111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMAXNMAL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMAXNMAL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMAXNMAL_16 {
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
                0b00111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDBFMINNMAL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDBFMINNMAL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDBFMINNMAL_16 {
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
                0b00111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFADD_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADD_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADD_16 {
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
                0b01111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAX_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAX_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAX_16 {
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
                0b01111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMIN_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMIN_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMIN_16 {
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
                0b01111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNM_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNM_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNM_16 {
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
                0b01111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNM_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNM_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNM_16 {
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
                0b01111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STFADD_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFADD_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFADD_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAX_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAX_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAX_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMIN_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMIN_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMIN_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAXNM_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAXNM_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAXNM_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMINNM_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMINNM_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMINNM_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFADDL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFADDL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFADDL_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAXL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAXL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAXL_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMINL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMINL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMINL_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAXNML_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAXNML_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAXNML_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMINNML_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMINNML_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMINNML_16 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod LDFADDL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADDL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADDL_16 {
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
                0b01111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXL_16 {
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
                0b01111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINL_16 {
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
                0b01111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNML_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNML_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNML_16 {
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
                0b01111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNML_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNML_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNML_16 {
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
                0b01111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFADDA_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADDA_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADDA_16 {
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
                0b01111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXA_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXA_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXA_16 {
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
                0b01111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINA_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINA_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINA_16 {
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
                0b01111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNMA_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNMA_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNMA_16 {
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
                0b01111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNMA_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNMA_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNMA_16 {
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
                0b01111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFADDAL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADDAL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADDAL_16 {
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
                0b01111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXAL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXAL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXAL_16 {
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
                0b01111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINAL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINAL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINAL_16 {
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
                0b01111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNMAL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNMAL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNMAL_16 {
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
                0b01111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNMAL_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNMAL_16 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNMAL_16 {
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
                0b01111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFADD_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADD_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADD_32 {
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
                0b10111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAX_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAX_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAX_32 {
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
                0b10111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMIN_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMIN_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMIN_32 {
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
                0b10111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNM_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNM_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNM_32 {
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
                0b10111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNM_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNM_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNM_32 {
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
                0b10111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STFADD_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFADD_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFADD_32 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAX_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAX_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAX_32 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMIN_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMIN_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMIN_32 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAXNM_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAXNM_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAXNM_32 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMINNM_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMINNM_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMINNM_32 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFADDL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFADDL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFADDL_32 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAXL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAXL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAXL_32 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMINL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMINL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMINL_32 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAXNML_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAXNML_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAXNML_32 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMINNML_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMINNML_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMINNML_32 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod LDFADDL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADDL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADDL_32 {
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
                0b10111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXL_32 {
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
                0b10111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINL_32 {
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
                0b10111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNML_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNML_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNML_32 {
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
                0b10111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNML_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNML_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNML_32 {
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
                0b10111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFADDA_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADDA_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADDA_32 {
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
                0b10111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXA_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXA_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXA_32 {
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
                0b10111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINA_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINA_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINA_32 {
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
                0b10111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNMA_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNMA_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNMA_32 {
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
                0b10111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNMA_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNMA_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNMA_32 {
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
                0b10111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFADDAL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADDAL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADDAL_32 {
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
                0b10111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXAL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXAL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXAL_32 {
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
                0b10111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINAL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINAL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINAL_32 {
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
                0b10111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNMAL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNMAL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNMAL_32 {
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
                0b10111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNMAL_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNMAL_32 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNMAL_32 {
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
                0b10111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFADD_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADD_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADD_64 {
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
                0b11111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAX_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAX_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAX_64 {
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
                0b11111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMIN_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMIN_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMIN_64 {
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
                0b11111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNM_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNM_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNM_64 {
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
                0b11111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNM_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNM_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNM_64 {
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
                0b11111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STFADD_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFADD_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFADD_64 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAX_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAX_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAX_64 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMIN_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMIN_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMIN_64 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAXNM_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAXNM_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAXNM_64 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMINNM_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMINNM_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMINNM_64 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFADDL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFADDL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFADDL_64 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAXL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAXL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAXL_64 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMINL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMINL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMINL_64 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b110100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMAXNML_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMAXNML_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMAXNML_64 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod STFMINNML_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STFMINNML_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl STFMINNML_64 {
        #[inline]
        pub fn new(
            Rs: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rs: Rs.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod LDFADDL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADDL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADDL_64 {
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
                0b11111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXL_64 {
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
                0b11111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINL_64 {
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
                0b11111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNML_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNML_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNML_64 {
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
                0b11111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNML_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNML_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNML_64 {
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
                0b11111100011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFADDA_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADDA_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADDA_64 {
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
                0b11111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXA_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXA_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXA_64 {
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
                0b11111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINA_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINA_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINA_64 {
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
                0b11111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNMA_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNMA_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNMA_64 {
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
                0b11111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNMA_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNMA_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNMA_64 {
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
                0b11111100101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFADDAL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFADDAL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFADDAL_64 {
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
                0b11111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXAL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXAL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXAL_64 {
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
                0b11111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINAL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINAL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINAL_64 {
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
                0b11111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b010100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMAXNMAL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMAXNMAL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMAXNMAL_64 {
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
                0b11111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDFMINNMAL_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDFMINNMAL_64 {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDFMINNMAL_64 {
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
                0b11111100111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b011100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
