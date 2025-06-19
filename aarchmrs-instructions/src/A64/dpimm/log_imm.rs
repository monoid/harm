/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_32_log_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AND_32_log_imm {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AND_32_log_imm {
        #[inline]
        pub fn new(
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001001000u32 << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ORR_32_log_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORR_32_log_imm {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORR_32_log_imm {
        #[inline]
        pub fn new(
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011001000u32 << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod EOR_32_log_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EOR_32_log_imm {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EOR_32_log_imm {
        #[inline]
        pub fn new(
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101001000u32 << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ANDS_32S_log_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ANDS_32S_log_imm {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ANDS_32S_log_imm {
        #[inline]
        pub fn new(
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111001000u32 << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod AND_64_log_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AND_64_log_imm {
        pub N: ::aarchmrs_types::BitValue<1>,
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AND_64_log_imm {
        #[inline]
        pub fn new(
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                N: N.into(),
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100100100u32 << 23u32
                    | u32::from(self.N) << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ORR_64_log_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORR_64_log_imm {
        pub N: ::aarchmrs_types::BitValue<1>,
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORR_64_log_imm {
        #[inline]
        pub fn new(
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                N: N.into(),
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101100100u32 << 23u32
                    | u32::from(self.N) << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod EOR_64_log_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EOR_64_log_imm {
        pub N: ::aarchmrs_types::BitValue<1>,
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EOR_64_log_imm {
        #[inline]
        pub fn new(
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                N: N.into(),
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110100100u32 << 23u32
                    | u32::from(self.N) << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ANDS_64S_log_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ANDS_64S_log_imm {
        pub N: ::aarchmrs_types::BitValue<1>,
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ANDS_64S_log_imm {
        #[inline]
        pub fn new(
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                N: N.into(),
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b111100100u32 << 23u32
                    | u32::from(self.N) << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
