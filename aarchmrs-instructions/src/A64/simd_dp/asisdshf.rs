/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SSHR_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SSHR_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SSHR_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                o1: o1.into(),
                o0: o0.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.o1) << 13u32
                    | u32::from(self.o0) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SSRA_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SSRA_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SSRA_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                o1: o1.into(),
                o0: o0.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.o1) << 13u32
                    | u32::from(self.o0) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SRSHR_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SRSHR_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SRSHR_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                o1: o1.into(),
                o0: o0.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.o1) << 13u32
                    | u32::from(self.o0) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SRSRA_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SRSRA_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SRSRA_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                o1: o1.into(),
                o0: o0.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.o1) << 13u32
                    | u32::from(self.o0) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SHL_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SHL_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SHL_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b010101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQSHL_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHL_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHL_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b011u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQSHRN_asisdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHRN_asisdshf_N {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHRN_asisdshf_N {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b1001u32 << 12u32
                    | u32::from(self.op) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQRSHRN_asisdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRSHRN_asisdshf_N {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRSHRN_asisdshf_N {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b1001u32 << 12u32
                    | u32::from(self.op) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SCVTF_asisdshf_C {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SCVTF_asisdshf_C {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SCVTF_asisdshf_C {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b111001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTZS_asisdshf_C {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTZS_asisdshf_C {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTZS_asisdshf_C {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod USHR_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USHR_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USHR_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                o1: o1.into(),
                o0: o0.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.o1) << 13u32
                    | u32::from(self.o0) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod USRA_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USRA_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USRA_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                o1: o1.into(),
                o0: o0.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.o1) << 13u32
                    | u32::from(self.o0) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod URSHR_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct URSHR_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl URSHR_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                o1: o1.into(),
                o0: o0.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.o1) << 13u32
                    | u32::from(self.o0) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod URSRA_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct URSRA_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl URSRA_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                o1: o1.into(),
                o0: o0.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.o1) << 13u32
                    | u32::from(self.o0) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SRI_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SRI_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SRI_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b010001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SLI_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SLI_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SLI_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b010101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQSHLU_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHLU_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHLU_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b011u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UQSHL_asisdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UQSHL_asisdshf_R {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UQSHL_asisdshf_R {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b011u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQSHRUN_asisdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHRUN_asisdshf_N {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHRUN_asisdshf_N {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b1000u32 << 12u32
                    | u32::from(self.op) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQRSHRUN_asisdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRSHRUN_asisdshf_N {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRSHRUN_asisdshf_N {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b1000u32 << 12u32
                    | u32::from(self.op) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UQSHRN_asisdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UQSHRN_asisdshf_N {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UQSHRN_asisdshf_N {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b1001u32 << 12u32
                    | u32::from(self.op) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UQRSHRN_asisdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UQRSHRN_asisdshf_N {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UQRSHRN_asisdshf_N {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b1001u32 << 12u32
                    | u32::from(self.op) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UCVTF_asisdshf_C {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UCVTF_asisdshf_C {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UCVTF_asisdshf_C {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b111001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTZU_asisdshf_C {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTZU_asisdshf_C {
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTZU_asisdshf_C {
        #[inline]
        pub fn new(
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
