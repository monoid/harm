/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MADD_32A_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MADD_32A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MADD_32A_dp_3src {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Ra: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Ra: Ra.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011011000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Ra) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MSUB_32A_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MSUB_32A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MSUB_32A_dp_3src {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Ra: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Ra: Ra.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011011000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Ra) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MADD_64A_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MADD_64A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MADD_64A_dp_3src {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Ra: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Ra: Ra.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Ra) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MSUB_64A_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MSUB_64A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MSUB_64A_dp_3src {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Ra: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Ra: Ra.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Ra) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SMADDL_64WA_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMADDL_64WA_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMADDL_64WA_dp_3src {
        #[inline]
        pub fn new(
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Ra: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                U: U.into(),
                Rm: Rm.into(),
                Ra: Ra.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | u32::from(self.U) << 23u32
                    | 0b01u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Ra) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SMSUBL_64WA_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMSUBL_64WA_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMSUBL_64WA_dp_3src {
        #[inline]
        pub fn new(
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Ra: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                U: U.into(),
                Rm: Rm.into(),
                Ra: Ra.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | u32::from(self.U) << 23u32
                    | 0b01u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Ra) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SMULH_64_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMULH_64_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMULH_64_dp_3src {
        #[inline]
        pub fn new(
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                U: U.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | u32::from(self.U) << 23u32
                    | 0b10u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b011111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MADDPT_64A_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MADDPT_64A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MADDPT_64A_dp_3src {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Ra: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Ra: Ra.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Ra) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MSUBPT_64A_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MSUBPT_64A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MSUBPT_64A_dp_3src {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Ra: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Ra: Ra.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Ra) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UMADDL_64WA_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMADDL_64WA_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMADDL_64WA_dp_3src {
        #[inline]
        pub fn new(
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Ra: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                U: U.into(),
                Rm: Rm.into(),
                Ra: Ra.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | u32::from(self.U) << 23u32
                    | 0b01u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Ra) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UMSUBL_64WA_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMSUBL_64WA_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMSUBL_64WA_dp_3src {
        #[inline]
        pub fn new(
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Ra: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                U: U.into(),
                Rm: Rm.into(),
                Ra: Ra.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | u32::from(self.U) << 23u32
                    | 0b01u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.Ra) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UMULH_64_dp_3src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMULH_64_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMULH_64_dp_3src {
        #[inline]
        pub fn new(
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                U: U.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | u32::from(self.U) << 23u32
                    | 0b10u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b011111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
