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
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b11u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b11u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1011u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            op: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                L,
                M,
                Rm,
                op,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.op.into_inner() << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            op: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                L,
                M,
                Rm,
                op,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.op.into_inner() << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b01u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b01u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                sz,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b01u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                sz,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b01u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                sz,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010111111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                L,
                M,
                Rm,
                S,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.S.into_inner() << 13u32
                    | 0b1u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                L,
                M,
                Rm,
                S,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.S.into_inner() << 13u32
                    | 0b1u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                sz,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011111111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
