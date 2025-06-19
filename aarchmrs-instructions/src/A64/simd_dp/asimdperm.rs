/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod UZP1_asimdperm_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UZP1_asimdperm_only {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UZP1_asimdperm_only {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
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
                    | 0b001110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.op) << 14u32
                    | 0b0110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod TRN1_asimdperm_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TRN1_asimdperm_only {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl TRN1_asimdperm_only {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
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
                    | 0b001110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.op) << 14u32
                    | 0b1010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ZIP1_asimdperm_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ZIP1_asimdperm_only {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ZIP1_asimdperm_only {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
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
                    | 0b001110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.op) << 14u32
                    | 0b1110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UZP2_asimdperm_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UZP2_asimdperm_only {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UZP2_asimdperm_only {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
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
                    | 0b001110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.op) << 14u32
                    | 0b0110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod TRN2_asimdperm_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TRN2_asimdperm_only {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl TRN2_asimdperm_only {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
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
                    | 0b001110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.op) << 14u32
                    | 0b1010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ZIP2_asimdperm_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ZIP2_asimdperm_only {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ZIP2_asimdperm_only {
        #[inline]
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                size: size.into(),
                Rm: Rm.into(),
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
                    | 0b001110u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.op) << 14u32
                    | 0b1110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
