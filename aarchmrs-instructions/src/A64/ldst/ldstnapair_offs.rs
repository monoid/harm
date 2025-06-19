/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STNP_32_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STNP_32_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STNP_32_ldstnapair_offs {
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
                0b0010100000u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDNP_32_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDNP_32_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDNP_32_ldstnapair_offs {
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
                0b0010100001u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STNP_S_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STNP_S_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STNP_S_ldstnapair_offs {
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
                0b0010110000u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDNP_S_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDNP_S_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDNP_S_ldstnapair_offs {
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
                0b0010110001u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STNP_D_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STNP_D_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STNP_D_ldstnapair_offs {
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
                0b0110110000u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDNP_D_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDNP_D_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDNP_D_ldstnapair_offs {
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
                0b0110110001u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STNP_64_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STNP_64_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STNP_64_ldstnapair_offs {
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
                0b1010100000u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDNP_64_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDNP_64_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDNP_64_ldstnapair_offs {
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
                0b1010100001u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STNP_Q_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STNP_Q_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STNP_Q_ldstnapair_offs {
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
                0b1010110000u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDNP_Q_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDNP_Q_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDNP_Q_ldstnapair_offs {
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
                0b1010110001u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STTNP_64_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTNP_64_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTNP_64_ldstnapair_offs {
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
                0b1110100000u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTNP_64_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTNP_64_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTNP_64_ldstnapair_offs {
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
                0b1110100001u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod STTNP_Q_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTNP_Q_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTNP_Q_ldstnapair_offs {
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
                0b1110110000u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDTNP_Q_ldstnapair_offs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTNP_Q_ldstnapair_offs {
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub Rt2: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTNP_Q_ldstnapair_offs {
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
                0b1110110001u32 << 22u32
                    | u32::from(self.imm7) << 15u32
                    | u32::from(self.Rt2) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
