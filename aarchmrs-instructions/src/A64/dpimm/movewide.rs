/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOVN_32_movewide {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVN_32_movewide {
        pub hw: ::aarchmrs_types::BitValue<1>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVN_32_movewide {
        #[inline]
        pub fn new(
            hw: impl Into<::aarchmrs_types::BitValue<1>>,
            imm16: impl Into<::aarchmrs_types::BitValue<16>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                hw: hw.into(),
                imm16: imm16.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001001010u32 << 22u32
                    | u32::from(self.hw) << 21u32
                    | u32::from(self.imm16) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MOVZ_32_movewide {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVZ_32_movewide {
        pub hw: ::aarchmrs_types::BitValue<1>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVZ_32_movewide {
        #[inline]
        pub fn new(
            hw: impl Into<::aarchmrs_types::BitValue<1>>,
            imm16: impl Into<::aarchmrs_types::BitValue<16>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                hw: hw.into(),
                imm16: imm16.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101001010u32 << 22u32
                    | u32::from(self.hw) << 21u32
                    | u32::from(self.imm16) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MOVK_32_movewide {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVK_32_movewide {
        pub hw: ::aarchmrs_types::BitValue<1>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVK_32_movewide {
        #[inline]
        pub fn new(
            hw: impl Into<::aarchmrs_types::BitValue<1>>,
            imm16: impl Into<::aarchmrs_types::BitValue<16>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                hw: hw.into(),
                imm16: imm16.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111001010u32 << 22u32
                    | u32::from(self.hw) << 21u32
                    | u32::from(self.imm16) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MOVN_64_movewide {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVN_64_movewide {
        pub hw: ::aarchmrs_types::BitValue<2>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVN_64_movewide {
        #[inline]
        pub fn new(
            hw: impl Into<::aarchmrs_types::BitValue<2>>,
            imm16: impl Into<::aarchmrs_types::BitValue<16>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                hw: hw.into(),
                imm16: imm16.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100100101u32 << 23u32
                    | u32::from(self.hw) << 21u32
                    | u32::from(self.imm16) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MOVZ_64_movewide {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVZ_64_movewide {
        pub hw: ::aarchmrs_types::BitValue<2>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVZ_64_movewide {
        #[inline]
        pub fn new(
            hw: impl Into<::aarchmrs_types::BitValue<2>>,
            imm16: impl Into<::aarchmrs_types::BitValue<16>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                hw: hw.into(),
                imm16: imm16.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110100101u32 << 23u32
                    | u32::from(self.hw) << 21u32
                    | u32::from(self.imm16) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MOVK_64_movewide {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVK_64_movewide {
        pub hw: ::aarchmrs_types::BitValue<2>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVK_64_movewide {
        #[inline]
        pub fn new(
            hw: impl Into<::aarchmrs_types::BitValue<2>>,
            imm16: impl Into<::aarchmrs_types::BitValue<16>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                hw: hw.into(),
                imm16: imm16.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b111100101u32 << 23u32
                    | u32::from(self.hw) << 21u32
                    | u32::from(self.imm16) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
