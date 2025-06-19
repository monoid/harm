/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SQDMLAL_asisdelem_L {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQDMLAL_asisdelem_L {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQDMLAL_asisdelem_L {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                o2: o2.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011111u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.o2) << 14u32
                    | 0b11u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQDMLSL_asisdelem_L {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQDMLSL_asisdelem_L {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQDMLSL_asisdelem_L {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                o2: o2.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011111u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.o2) << 14u32
                    | 0b11u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQDMULL_asisdelem_L {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQDMULL_asisdelem_L {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQDMULL_asisdelem_L {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011111u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1011u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQDMULH_asisdelem_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQDMULH_asisdelem_R {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQDMULH_asisdelem_R {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                op: op.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011111u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQRDMULH_asisdelem_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRDMULH_asisdelem_R {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRDMULH_asisdelem_R {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                op: op.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011111u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.op) << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMLA_asisdelem_RH_H {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLA_asisdelem_RH_H {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLA_asisdelem_RH_H {
        #[inline]
        pub fn new(
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                o2: o2.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101111100u32 << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.o2) << 14u32
                    | 0b01u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMLS_asisdelem_RH_H {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLS_asisdelem_RH_H {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLS_asisdelem_RH_H {
        #[inline]
        pub fn new(
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                o2: o2.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101111100u32 << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.o2) << 14u32
                    | 0b01u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMUL_asisdelem_RH_H {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMUL_asisdelem_RH_H {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMUL_asisdelem_RH_H {
        #[inline]
        pub fn new(
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101111100u32 << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1001u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMLA_asisdelem_R_SD {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLA_asisdelem_R_SD {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLA_asisdelem_R_SD {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                o2: o2.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111111u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.o2) << 14u32
                    | 0b01u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMLS_asisdelem_R_SD {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLS_asisdelem_R_SD {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLS_asisdelem_R_SD {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            o2: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                o2: o2.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111111u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.o2) << 14u32
                    | 0b01u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMUL_asisdelem_R_SD {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMUL_asisdelem_R_SD {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMUL_asisdelem_R_SD {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111111u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1001u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQRDMLAH_asisdelem_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRDMLAH_asisdelem_R {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRDMLAH_asisdelem_R {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                S: S.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111111u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SQRDMLSH_asisdelem_R {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRDMLSH_asisdelem_R {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRDMLSH_asisdelem_R {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                S: S.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111111u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.S) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMULX_asisdelem_RH_H {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMULX_asisdelem_RH_H {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMULX_asisdelem_RH_H {
        #[inline]
        pub fn new(
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111111100u32 << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1001u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMULX_asisdelem_R_SD {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMULX_asisdelem_R_SD {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMULX_asisdelem_R_SD {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            L: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<4>>,
            H: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                L: L.into(),
                M: M.into(),
                Rm: Rm.into(),
                H: H.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111111u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | u32::from(self.L) << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1001u32 << 12u32
                    | u32::from(self.H) << 11u32
                    | 0b0u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
