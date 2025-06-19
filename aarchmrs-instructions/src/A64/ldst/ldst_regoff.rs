/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRB_32B_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STRB_32B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STRB_32B_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STRB_32BL_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STRB_32BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STRB_32BL_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b011u32 << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRB_32B_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRB_32B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRB_32B_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRB_32BL_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRB_32BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRB_32BL_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b011u32 << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSB_64B_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_64B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_64B_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000101u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSB_64BL_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_64BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_64BL_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000101u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b011u32 << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSB_32B_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_32B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_32B_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000111u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSB_32BL_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_32BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_32BL_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000111u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b011u32 << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_B_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_B_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_BL_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_BL_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b011u32 << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_B_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_B_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_BL_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_BL_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b011u32 << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_Q_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_Q_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_Q_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100101u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_Q_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_Q_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_Q_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100111u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STRH_32_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STRH_32_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STRH_32_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRH_32_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRH_32_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRH_32_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSH_64_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSH_64_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSH_64_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000101u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSH_32_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSH_32_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSH_32_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000111u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_H_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_H_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_H_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_H_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_H_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_H_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_32_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_32_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_32_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_32_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_32_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_32_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSW_64_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSW_64_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSW_64_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000101u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_S_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_S_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_S_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_S_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_S_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_S_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_64_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_64_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_64_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_64_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_64_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_64_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod PRFM_P_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PRFM_P_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl PRFM_P_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000101u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod RPRFM_R_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RPRFM_R_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RPRFM_R_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000101u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_D_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_D_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_D_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_D_ldst_regoff {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_D_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_D_ldst_regoff {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.S) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
