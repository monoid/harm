/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDRAA_64_ldst_pac {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRAA_64_ldst_pac {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRAA_64_ldst_pac {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            imm9: impl Into<::aarchmrs_types::BitValue<9>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                imm9: imm9.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b111110000u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm9) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRAA_64W_ldst_pac {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRAA_64W_ldst_pac {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRAA_64W_ldst_pac {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            imm9: impl Into<::aarchmrs_types::BitValue<9>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                imm9: imm9.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b111110000u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm9) << 12u32
                    | 0b11u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRAB_64_ldst_pac {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRAB_64_ldst_pac {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRAB_64_ldst_pac {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            imm9: impl Into<::aarchmrs_types::BitValue<9>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                imm9: imm9.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b111110001u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm9) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod LDRAB_64W_ldst_pac {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRAB_64W_ldst_pac {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRAB_64W_ldst_pac {
        #[inline]
        pub fn new(
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            imm9: impl Into<::aarchmrs_types::BitValue<9>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                S: S.into(),
                imm9: imm9.into(),
                Rn: Rn.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b111110001u32 << 23u32
                    | u32::from(self.S) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm9) << 12u32
                    | 0b11u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
