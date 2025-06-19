/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADDP_asisdpair_only {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDP_asisdpair_only {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDP_asisdpair_only {
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
                0b0101111011110001101110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMAXNMP_asisdpair_only_H {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMAXNMP_asisdpair_only_H {
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMAXNMP_asisdpair_only_H {
        #[inline]
        pub fn new(
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o1: o1.into(),
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.o1) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b110000110010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FADDP_asisdpair_only_H {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FADDP_asisdpair_only_H {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FADDP_asisdpair_only_H {
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
                    | 0b110000110110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMAXP_asisdpair_only_H {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMAXP_asisdpair_only_H {
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMAXP_asisdpair_only_H {
        #[inline]
        pub fn new(
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o1: o1.into(),
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.o1) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b110000111110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMINNMP_asisdpair_only_H {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMINNMP_asisdpair_only_H {
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMINNMP_asisdpair_only_H {
        #[inline]
        pub fn new(
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o1: o1.into(),
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.o1) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b110000110010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMINP_asisdpair_only_H {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMINP_asisdpair_only_H {
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMINP_asisdpair_only_H {
        #[inline]
        pub fn new(
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o1: o1.into(),
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011110u32 << 24u32
                    | u32::from(self.o1) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b110000111110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMAXNMP_asisdpair_only_SD {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMAXNMP_asisdpair_only_SD {
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMAXNMP_asisdpair_only_SD {
        #[inline]
        pub fn new(
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o1: o1.into(),
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.o1) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b110000110010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FADDP_asisdpair_only_SD {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FADDP_asisdpair_only_SD {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FADDP_asisdpair_only_SD {
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
                    | 0b110000110110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMAXP_asisdpair_only_SD {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMAXP_asisdpair_only_SD {
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMAXP_asisdpair_only_SD {
        #[inline]
        pub fn new(
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o1: o1.into(),
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.o1) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b110000111110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMINNMP_asisdpair_only_SD {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMINNMP_asisdpair_only_SD {
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMINNMP_asisdpair_only_SD {
        #[inline]
        pub fn new(
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o1: o1.into(),
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.o1) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b110000110010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMINP_asisdpair_only_SD {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMINP_asisdpair_only_SD {
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMINP_asisdpair_only_SD {
        #[inline]
        pub fn new(
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o1: o1.into(),
                sz: sz.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111110u32 << 24u32
                    | u32::from(self.o1) << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b110000111110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
