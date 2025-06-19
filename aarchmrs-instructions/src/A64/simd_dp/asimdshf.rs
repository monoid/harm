/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SSHR_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SSHR_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SSHR_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
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
pub mod SSRA_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SSRA_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SSRA_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
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
pub mod SRSHR_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SRSHR_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SRSHR_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
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
pub mod SRSRA_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SRSRA_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SRSRA_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
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
pub mod SHL_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SHL_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SHL_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b010101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQSHL_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHL_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHL_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
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
pub mod SHRN_asimdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SHRN_asimdshf_N {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
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
pub mod RSHRN_asimdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RSHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl RSHRN_asimdshf_N {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
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
pub mod SQSHRN_asimdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHRN_asimdshf_N {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
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
pub mod SQRSHRN_asimdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRSHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRSHRN_asimdshf_N {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
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
pub mod SSHLL_asimdshf_L {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SSHLL_asimdshf_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SSHLL_asimdshf_L {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b101001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SCVTF_asimdshf_C {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SCVTF_asimdshf_C {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SCVTF_asimdshf_C {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b111001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTZS_asimdshf_C {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTZS_asimdshf_C {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTZS_asimdshf_C {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b0011110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod USHR_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USHR_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USHR_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
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
pub mod USRA_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USRA_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USRA_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
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
pub mod URSHR_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct URSHR_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl URSHR_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
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
pub mod URSRA_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct URSRA_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl URSRA_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
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
pub mod SRI_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SRI_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SRI_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b010001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SLI_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SLI_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SLI_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b010101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQSHLU_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHLU_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHLU_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
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
pub mod UQSHL_asimdshf_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UQSHL_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UQSHL_asimdshf_R {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
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
pub mod SQSHRUN_asimdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHRUN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHRUN_asimdshf_N {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
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
pub mod SQRSHRUN_asimdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRSHRUN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRSHRUN_asimdshf_N {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
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
pub mod UQSHRN_asimdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UQSHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UQSHRN_asimdshf_N {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
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
pub mod UQRSHRN_asimdshf_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UQRSHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UQRSHRN_asimdshf_N {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
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
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
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
pub mod USHLL_asimdshf_L {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USHLL_asimdshf_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USHLL_asimdshf_L {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b101001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UCVTF_asimdshf_C {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UCVTF_asimdshf_C {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UCVTF_asimdshf_C {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b111001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTZU_asimdshf_C {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTZU_asimdshf_C {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTZU_asimdshf_C {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            immh: impl Into<::aarchmrs_types::BitValue<4>>,
            immb: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                immh: immh.into(),
                immb: immb.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b1011110u32 << 23u32
                    | u32::from(self.immh) << 19u32
                    | u32::from(self.immb) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
