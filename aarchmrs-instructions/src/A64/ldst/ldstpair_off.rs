/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STP_32_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STP_32_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STP_32_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010100100u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDP_32_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDP_32_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDP_32_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010100101u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STP_S_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STP_S_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STP_S_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010110100u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDP_S_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDP_S_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDP_S_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010110101u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STGP_64_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STGP_64_ldstpair_off {
        pub simm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STGP_64_ldstpair_off {
        #[inline]
        pub fn new(
            simm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                simm7: simm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110100100u32 << 22u32
                    | u32::from(self.simm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDPSW_64_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDPSW_64_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDPSW_64_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110100101u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STP_D_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STP_D_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STP_D_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110110100u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDP_D_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDP_D_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDP_D_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110110101u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STP_64_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STP_64_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STP_64_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010100100u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDP_64_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDP_64_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDP_64_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010100101u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STP_Q_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STP_Q_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STP_Q_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010110100u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDP_Q_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDP_Q_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDP_Q_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010110101u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STTP_64_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTP_64_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTP_64_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110100100u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTP_64_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTP_64_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTP_64_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110100101u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STTP_Q_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTP_Q_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTP_Q_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110110100u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTP_Q_ldstpair_off {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTP_Q_ldstpair_off {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTP_Q_ldstpair_off {
        #[inline]
        pub fn new(
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm7: imm7.into(),
                Rt2: Rt2.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110110101u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
