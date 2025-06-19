/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STP_32_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STP_32_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STP_32_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010100110u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDP_32_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDP_32_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDP_32_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010100111u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STP_S_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STP_S_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STP_S_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010110110u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDP_S_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDP_S_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDP_S_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010110111u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STGP_64_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STGP_64_ldstpair_pre {
        pub simm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STGP_64_ldstpair_pre {
        #[inline]
        pub const fn new(
            simm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { simm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110100110u32 << 22u32
                    | self.simm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDPSW_64_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDPSW_64_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDPSW_64_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110100111u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STP_D_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STP_D_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STP_D_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110110110u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDP_D_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDP_D_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDP_D_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110110111u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STP_64_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STP_64_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STP_64_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010100110u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDP_64_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDP_64_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDP_64_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010100111u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STP_Q_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STP_Q_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STP_Q_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010110110u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDP_Q_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDP_Q_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDP_Q_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010110111u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STTP_64_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTP_64_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTP_64_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110100110u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDTP_64_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTP_64_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTP_64_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110100111u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STTP_Q_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTP_Q_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTP_Q_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110110110u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDTP_Q_ldstpair_pre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTP_Q_ldstpair_pre {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTP_Q_ldstpair_pre {
        #[inline]
        pub const fn new(
            imm7: ::aarchmrs_types::BitValue<7>,
            Rt2: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm7, Rt2, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110110111u32 << 22u32
                    | self.imm7.into_inner() << 15u32
                    | self.Rt2.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
