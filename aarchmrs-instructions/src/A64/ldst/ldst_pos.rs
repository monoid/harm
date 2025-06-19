/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRB_32_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STRB_32_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STRB_32_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011100100u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRB_32_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRB_32_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRB_32_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011100101u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSB_64_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_64_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_64_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011100110u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSB_32_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_32_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_32_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011100111u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_B_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_B_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_B_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011110100u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_B_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_B_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_B_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011110101u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_Q_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_Q_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_Q_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011110110u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_Q_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_Q_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_Q_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011110111u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STRH_32_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STRH_32_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STRH_32_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111100100u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRH_32_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRH_32_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRH_32_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111100101u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSH_64_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSH_64_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSH_64_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111100110u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSH_32_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSH_32_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSH_32_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111100111u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_H_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_H_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_H_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111110100u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_H_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_H_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_H_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111110101u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_32_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_32_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_32_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1011100100u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_32_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_32_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_32_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1011100101u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRSW_64_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSW_64_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSW_64_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1011100110u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_S_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_S_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_S_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1011110100u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_S_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_S_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_S_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1011110101u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_64_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_64_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_64_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1111100100u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_64_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_64_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_64_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1111100101u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod PRFM_P_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PRFM_P_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl PRFM_P_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1111100110u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STR_D_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_D_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_D_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1111110100u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDR_D_ldst_pos {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_D_ldst_pos {
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_D_ldst_pos {
        #[inline]
        pub fn new(
            imm12: impl Into<::aarchmrs_types::BitValue<12>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm12: imm12.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1111110101u32 << 22u32
                    | u32::from(self.imm12) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
