/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SUQADD_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUQADD_asisdmisc_R {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUQADD_asisdmisc_R {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b100000001110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQABS_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQABS_asisdmisc_R {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQABS_asisdmisc_R {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b100000011110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CMGT_asisdmisc_Z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CMGT_asisdmisc_Z {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CMGT_asisdmisc_Z {
        #[inline]
        pub fn new(
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101111011100000100u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CMEQ_asisdmisc_Z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CMEQ_asisdmisc_Z {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CMEQ_asisdmisc_Z {
        #[inline]
        pub fn new(
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101111011100000100u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CMLT_asisdmisc_Z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CMLT_asisdmisc_Z {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CMLT_asisdmisc_Z {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101111011100000101010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ABS_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ABS_asisdmisc_R {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ABS_asisdmisc_R {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101111011100000101110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQXTN_asisdmisc_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQXTN_asisdmisc_N {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQXTN_asisdmisc_N {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b100001010010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTNS_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTNS_asisdmisc_R {
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTNS_asisdmisc_R {
        #[inline]
        pub fn new(
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o2: o2.into(),
                sz: sz.into(),
                o1: o1.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.o2) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001101u32 << 13u32
                    | u32::from(self.o1) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTMS_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTMS_asisdmisc_R {
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTMS_asisdmisc_R {
        #[inline]
        pub fn new(
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o2: o2.into(),
                sz: sz.into(),
                o1: o1.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.o2) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001101u32 << 13u32
                    | u32::from(self.o1) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTAS_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTAS_asisdmisc_R {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTAS_asisdmisc_R {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111100u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001110010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SCVTF_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SCVTF_asisdmisc_R {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SCVTF_asisdmisc_R {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111100u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001110110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCMGT_asisdmisc_FZ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMGT_asisdmisc_FZ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCMGT_asisdmisc_FZ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111101u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100000110u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCMEQ_asisdmisc_FZ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMEQ_asisdmisc_FZ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCMEQ_asisdmisc_FZ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111101u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100000110u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCMLT_asisdmisc_FZ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMLT_asisdmisc_FZ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCMLT_asisdmisc_FZ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111101u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100000111010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTPS_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTPS_asisdmisc_R {
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTPS_asisdmisc_R {
        #[inline]
        pub fn new(
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o2: o2.into(),
                sz: sz.into(),
                o1: o1.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.o2) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001101u32 << 13u32
                    | u32::from(self.o1) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTZS_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTZS_asisdmisc_R {
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTZS_asisdmisc_R {
        #[inline]
        pub fn new(
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o2: o2.into(),
                sz: sz.into(),
                o1: o1.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.o2) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001101u32 << 13u32
                    | u32::from(self.o1) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FRECPE_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRECPE_asisdmisc_R {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRECPE_asisdmisc_R {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111101u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001110110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FRECPX_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRECPX_asisdmisc_R {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRECPX_asisdmisc_R {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111101u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001111110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod USQADD_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USQADD_asisdmisc_R {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USQADD_asisdmisc_R {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b100000001110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQNEG_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQNEG_asisdmisc_R {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQNEG_asisdmisc_R {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b100000011110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CMGE_asisdmisc_Z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CMGE_asisdmisc_Z {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CMGE_asisdmisc_Z {
        #[inline]
        pub fn new(
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111111011100000100u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CMLE_asisdmisc_Z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CMLE_asisdmisc_Z {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CMLE_asisdmisc_Z {
        #[inline]
        pub fn new(
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111111011100000100u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod NEG_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct NEG_asisdmisc_R {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl NEG_asisdmisc_R {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111111011100000101110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQXTUN_asisdmisc_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQXTUN_asisdmisc_N {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQXTUN_asisdmisc_N {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b100001001010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UQXTN_asisdmisc_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UQXTN_asisdmisc_N {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UQXTN_asisdmisc_N {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b100001010010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTXN_asisdmisc_N {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTXN_asisdmisc_N {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTXN_asisdmisc_N {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111111001100001011010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTNU_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTNU_asisdmisc_R {
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTNU_asisdmisc_R {
        #[inline]
        pub fn new(
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o2: o2.into(),
                sz: sz.into(),
                o1: o1.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.o2) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001101u32 << 13u32
                    | u32::from(self.o1) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTMU_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTMU_asisdmisc_R {
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTMU_asisdmisc_R {
        #[inline]
        pub fn new(
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o2: o2.into(),
                sz: sz.into(),
                o1: o1.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.o2) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001101u32 << 13u32
                    | u32::from(self.o1) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTAU_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTAU_asisdmisc_R {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTAU_asisdmisc_R {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111100u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001110010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UCVTF_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UCVTF_asisdmisc_R {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UCVTF_asisdmisc_R {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111100u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001110110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCMGE_asisdmisc_FZ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMGE_asisdmisc_FZ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCMGE_asisdmisc_FZ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111101u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100000110u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCMLE_asisdmisc_FZ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMLE_asisdmisc_FZ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCMLE_asisdmisc_FZ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111101u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100000110u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTPU_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTPU_asisdmisc_R {
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTPU_asisdmisc_R {
        #[inline]
        pub fn new(
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o2: o2.into(),
                sz: sz.into(),
                o1: o1.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.o2) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001101u32 << 13u32
                    | u32::from(self.o1) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCVTZU_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTZU_asisdmisc_R {
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTZU_asisdmisc_R {
        #[inline]
        pub fn new(
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o2: o2.into(),
                sz: sz.into(),
                o1: o1.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.o2) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001101u32 << 13u32
                    | u32::from(self.o1) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FRSQRTE_asisdmisc_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRSQRTE_asisdmisc_R {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRSQRTE_asisdmisc_R {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111101u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b100001110110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
