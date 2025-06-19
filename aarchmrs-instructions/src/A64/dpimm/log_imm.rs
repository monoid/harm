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
        pub const fn new(
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immr, imms, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001001000u32 << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immr, imms, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011001000u32 << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immr, imms, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101001000u32 << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immr, imms, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111001000u32 << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            N: ::aarchmrs_types::BitValue<1>,
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                N,
                immr,
                imms,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100100100u32 << 23u32
                    | self.N.into_inner() << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            N: ::aarchmrs_types::BitValue<1>,
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                N,
                immr,
                imms,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101100100u32 << 23u32
                    | self.N.into_inner() << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            N: ::aarchmrs_types::BitValue<1>,
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                N,
                immr,
                imms,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110100100u32 << 23u32
                    | self.N.into_inner() << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            N: ::aarchmrs_types::BitValue<1>,
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                N,
                immr,
                imms,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b111100100u32 << 23u32
                    | self.N.into_inner() << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
