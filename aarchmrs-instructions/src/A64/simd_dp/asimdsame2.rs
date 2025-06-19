/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SDOT_asimdsame2_D {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SDOT_asimdsame2_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SDOT_asimdsame2_D {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b001110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b100101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTN_asimdsame2_H {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTN_asimdsame2_H {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTN_asimdsame2_H {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b001110000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FDOT_asimdsame2_DD {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FDOT_asimdsame2_DD {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FDOT_asimdsame2_DD {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b001110000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTN_asimdsame2_D {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTN_asimdsame2_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTN_asimdsame2_D {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b001110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FDOT_asimdsame2_D {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FDOT_asimdsame2_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FDOT_asimdsame2_D {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b001110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod USDOT_asimdsame2_D {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USDOT_asimdsame2_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USDOT_asimdsame2_D {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b001110100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b100111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQRDMLAH_asimdsame2_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRDMLAH_asimdsame2_only {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRDMLAH_asimdsame2_only {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b101110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1000u32 << 12u32
                    | u32::from(self.S) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQRDMLSH_asimdsame2_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRDMLSH_asimdsame2_only {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRDMLSH_asimdsame2_only {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
                S: S.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b101110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1000u32 << 12u32
                    | u32::from(self.S) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UDOT_asimdsame2_D {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UDOT_asimdsame2_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UDOT_asimdsame2_D {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b101110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b100101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCMLA_asimdsame2_C {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMLA_asimdsame2_C {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub rot: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCMLA_asimdsame2_C {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            rot: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
                rot: rot.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b101110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.rot) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCADD_asimdsame2_C {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCADD_asimdsame2_C {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub rot: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCADD_asimdsame2_C {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            rot: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
                rot: rot.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b101110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.rot) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BFDOT_asimdsame2_D {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BFDOT_asimdsame2_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BFDOT_asimdsame2_D {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b101110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BFMLAL_asimdsame2_F_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BFMLAL_asimdsame2_F_ {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BFMLAL_asimdsame2_F_ {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b101110110u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMLALLBB_asimdsame2_G {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALLBB_asimdsame2_G {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALLBB_asimdsame2_G {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001110000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMLALLBT_asimdsame2_G {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALLBT_asimdsame2_G {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALLBT_asimdsame2_G {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMLALB_asimdsame2_J {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALB_asimdsame2_J {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALB_asimdsame2_J {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001110110u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMLALLTB_asimdsame2_G {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALLTB_asimdsame2_G {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALLTB_asimdsame2_G {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001110000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMLALLTT_asimdsame2_G {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALLTT_asimdsame2_G {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALLTT_asimdsame2_G {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SMMLA_asimdsame2_G {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMMLA_asimdsame2_G {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMMLA_asimdsame2_G {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            B: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                B: B.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001110100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1010u32 << 12u32
                    | u32::from(self.B) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod USMMLA_asimdsame2_G {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USMMLA_asimdsame2_G {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USMMLA_asimdsame2_G {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            B: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                B: B.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001110100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1010u32 << 12u32
                    | u32::from(self.B) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMLALT_asimdsame2_J {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALT_asimdsame2_J {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALT_asimdsame2_J {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001110110u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMMLA_asimd_FP8FP16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMMLA_asimd_FP8FP16 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMMLA_asimd_FP8FP16 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01101110000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111011u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BFMMLA_asimdsame2_E {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BFMMLA_asimdsame2_E {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BFMMLA_asimdsame2_E {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01101110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111011u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMMLA_asimd_FP8FP32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMMLA_asimd_FP8FP32 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMMLA_asimd_FP8FP32 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01101110100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b111011u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UMMLA_asimdsame2_G {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMMLA_asimdsame2_G {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMMLA_asimdsame2_G {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            B: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                B: B.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01101110100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1010u32 << 12u32
                    | u32::from(self.B) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
