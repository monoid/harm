/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FMULX_asisdsamefp16_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMULX_asisdsamefp16_only {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMULX_asisdsamefp16_only {
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
                0b01011110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b000111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCMEQ_asisdsamefp16_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMEQ_asisdsamefp16_only {
        pub E: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub ac: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCMEQ_asisdsamefp16_only {
        #[inline]
        pub fn new(
            E: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            ac: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                E: E.into(),
                Rm: Rm.into(),
                ac: ac.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.E) << 23u32
                    | 0b10u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0010u32 << 12u32
                    | u32::from(self.ac) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FRECPS_asisdsamefp16_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRECPS_asisdsamefp16_only {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRECPS_asisdsamefp16_only {
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
                0b01011110010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b001111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FRSQRTS_asisdsamefp16_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRSQRTS_asisdsamefp16_only {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRSQRTS_asisdsamefp16_only {
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
                0b01011110110u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b001111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCMGE_asisdsamefp16_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMGE_asisdsamefp16_only {
        pub E: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub ac: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCMGE_asisdsamefp16_only {
        #[inline]
        pub fn new(
            E: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            ac: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                E: E.into(),
                Rm: Rm.into(),
                ac: ac.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.E) << 23u32
                    | 0b10u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0010u32 << 12u32
                    | u32::from(self.ac) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FACGE_asisdsamefp16_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FACGE_asisdsamefp16_only {
        pub E: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub ac: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FACGE_asisdsamefp16_only {
        #[inline]
        pub fn new(
            E: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            ac: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                E: E.into(),
                Rm: Rm.into(),
                ac: ac.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.E) << 23u32
                    | 0b10u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0010u32 << 12u32
                    | u32::from(self.ac) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FABD_asisdsamefp16_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FABD_asisdsamefp16_only {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FABD_asisdsamefp16_only {
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
                0b01111110110u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b000101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FCMGT_asisdsamefp16_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMGT_asisdsamefp16_only {
        pub E: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub ac: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCMGT_asisdsamefp16_only {
        #[inline]
        pub fn new(
            E: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            ac: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                E: E.into(),
                Rm: Rm.into(),
                ac: ac.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.E) << 23u32
                    | 0b10u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0010u32 << 12u32
                    | u32::from(self.ac) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FACGT_asisdsamefp16_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FACGT_asisdsamefp16_only {
        pub E: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub ac: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FACGT_asisdsamefp16_only {
        #[inline]
        pub fn new(
            E: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            ac: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                E: E.into(),
                Rm: Rm.into(),
                ac: ac.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.E) << 23u32
                    | 0b10u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0010u32 << 12u32
                    | u32::from(self.ac) << 11u32
                    | 0b1u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
