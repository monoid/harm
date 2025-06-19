/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CCMN_32_condcmp_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CCMN_32_condcmp_reg {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl CCMN_32_condcmp_reg {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                nzcv: nzcv.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111010010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.nzcv) << 0u32,
            )
        }
    }
}
pub mod CCMP_32_condcmp_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CCMP_32_condcmp_reg {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl CCMP_32_condcmp_reg {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                nzcv: nzcv.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111010010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.nzcv) << 0u32,
            )
        }
    }
}
pub mod CCMN_64_condcmp_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CCMN_64_condcmp_reg {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl CCMN_64_condcmp_reg {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                nzcv: nzcv.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111010010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.nzcv) << 0u32,
            )
        }
    }
}
pub mod CCMP_64_condcmp_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CCMP_64_condcmp_reg {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl CCMP_64_condcmp_reg {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                nzcv: nzcv.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111010010u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.nzcv) << 0u32,
            )
        }
    }
}
