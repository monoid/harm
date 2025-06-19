/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_32_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AND_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AND_32_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BIC_32_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BIC_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BIC_32_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ORR_32_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORR_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORR_32_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00101010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ORN_32_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORN_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORN_32_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00101010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod EOR_32_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EOR_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EOR_32_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod EON_32_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EON_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EON_32_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ANDS_32_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ANDS_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ANDS_32_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01101010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BICS_32_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BICS_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BICS_32_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01101010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod AND_64_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AND_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AND_64_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BIC_64_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BIC_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BIC_64_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ORR_64_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORR_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORR_64_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10101010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ORN_64_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORN_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORN_64_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10101010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod EOR_64_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EOR_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EOR_64_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod EON_64_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EON_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EON_64_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ANDS_64_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ANDS_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ANDS_64_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11101010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BICS_64_log_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BICS_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BICS_64_log_shift {
        #[inline]
        pub fn new(
            shift: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                shift: shift.into(),
                Rm: Rm.into(),
                imm6: imm6.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11101010u32 << 24u32
                    | u32::from(self.shift) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.imm6) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
