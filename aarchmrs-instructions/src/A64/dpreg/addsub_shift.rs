/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_32_addsub_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADD_32_addsub_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADD_32_addsub_shift {
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
                0b00001011u32 << 24u32
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
pub mod ADDS_32_addsub_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDS_32_addsub_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDS_32_addsub_shift {
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
                0b00101011u32 << 24u32
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
pub mod SUB_32_addsub_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUB_32_addsub_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUB_32_addsub_shift {
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
                0b01001011u32 << 24u32
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
pub mod SUBS_32_addsub_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUBS_32_addsub_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUBS_32_addsub_shift {
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
                0b01101011u32 << 24u32
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
pub mod ADD_64_addsub_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADD_64_addsub_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADD_64_addsub_shift {
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
                0b10001011u32 << 24u32
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
pub mod ADDS_64_addsub_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDS_64_addsub_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDS_64_addsub_shift {
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
                0b10101011u32 << 24u32
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
pub mod SUB_64_addsub_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUB_64_addsub_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUB_64_addsub_shift {
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
                0b11001011u32 << 24u32
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
pub mod SUBS_64_addsub_shift {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUBS_64_addsub_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUBS_64_addsub_shift {
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
                0b11101011u32 << 24u32
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
