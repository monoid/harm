/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SBFM_32M_bitfield {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SBFM_32M_bitfield {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SBFM_32M_bitfield {
        #[inline]
        pub fn new(
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001001100u32 << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BFM_32M_bitfield {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BFM_32M_bitfield {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BFM_32M_bitfield {
        #[inline]
        pub fn new(
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011001100u32 << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UBFM_32M_bitfield {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UBFM_32M_bitfield {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UBFM_32M_bitfield {
        #[inline]
        pub fn new(
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101001100u32 << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SBFM_64M_bitfield {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SBFM_64M_bitfield {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SBFM_64M_bitfield {
        #[inline]
        pub fn new(
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1001001101u32 << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BFM_64M_bitfield {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BFM_64M_bitfield {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BFM_64M_bitfield {
        #[inline]
        pub fn new(
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1011001101u32 << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod UBFM_64M_bitfield {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UBFM_64M_bitfield {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UBFM_64M_bitfield {
        #[inline]
        pub fn new(
            immr: impl Into<::aarchmrs_types::BitValue<6>>,
            imms: impl Into<::aarchmrs_types::BitValue<6>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                immr: immr.into(),
                imms: imms.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101001101u32 << 22u32
                    | u32::from(self.immr) << 16u32
                    | u32::from(self.imms) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
