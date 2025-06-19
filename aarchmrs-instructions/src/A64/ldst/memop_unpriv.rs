/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDTADD_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTADD_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTADD_32_memop_unpriv {
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
                0b00011001001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTCLR_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTCLR_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTCLR_32_memop_unpriv {
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
                0b00011001001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTSET_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTSET_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTSET_32_memop_unpriv {
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
                0b00011001001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPT_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPT_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPT_32_memop_unpriv {
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
                0b00011001001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTADDL_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTADDL_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTADDL_32_memop_unpriv {
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
                0b00011001011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTCLRL_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTCLRL_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTCLRL_32_memop_unpriv {
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
                0b00011001011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTSETL_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTSETL_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTSETL_32_memop_unpriv {
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
                0b00011001011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPTL_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPTL_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPTL_32_memop_unpriv {
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
                0b00011001011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTADDA_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTADDA_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTADDA_32_memop_unpriv {
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
                0b00011001101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTCLRA_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTCLRA_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTCLRA_32_memop_unpriv {
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
                0b00011001101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTSETA_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTSETA_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTSETA_32_memop_unpriv {
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
                0b00011001101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPTA_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPTA_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPTA_32_memop_unpriv {
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
                0b00011001101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTADDAL_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTADDAL_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTADDAL_32_memop_unpriv {
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
                0b00011001111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTCLRAL_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTCLRAL_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTCLRAL_32_memop_unpriv {
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
                0b00011001111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTSETAL_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTSETAL_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTSETAL_32_memop_unpriv {
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
                0b00011001111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPTAL_32_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPTAL_32_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPTAL_32_memop_unpriv {
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
                0b00011001111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTADD_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTADD_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTADD_64_memop_unpriv {
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
                0b01011001001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTCLR_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTCLR_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTCLR_64_memop_unpriv {
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
                0b01011001001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTSET_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTSET_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTSET_64_memop_unpriv {
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
                0b01011001001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPT_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPT_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPT_64_memop_unpriv {
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
                0b01011001001u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTADDL_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTADDL_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTADDL_64_memop_unpriv {
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
                0b01011001011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTCLRL_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTCLRL_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTCLRL_64_memop_unpriv {
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
                0b01011001011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTSETL_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTSETL_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTSETL_64_memop_unpriv {
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
                0b01011001011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPTL_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPTL_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPTL_64_memop_unpriv {
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
                0b01011001011u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTADDA_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTADDA_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTADDA_64_memop_unpriv {
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
                0b01011001101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTCLRA_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTCLRA_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTCLRA_64_memop_unpriv {
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
                0b01011001101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTSETA_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTSETA_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTSETA_64_memop_unpriv {
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
                0b01011001101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPTA_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPTA_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPTA_64_memop_unpriv {
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
                0b01011001101u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTADDAL_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTADDAL_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTADDAL_64_memop_unpriv {
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
                0b01011001111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTCLRAL_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTCLRAL_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTCLRAL_64_memop_unpriv {
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
                0b01011001111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b000101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTSETAL_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTSETAL_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTSETAL_64_memop_unpriv {
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
                0b01011001111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b001101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SWPTAL_64_memop_unpriv {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SWPTAL_64_memop_unpriv {
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SWPTAL_64_memop_unpriv {
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
                0b01011001111u32 << 21u32
                    | u32::from(self.Rs) << 16u32
                    | 0b100001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
