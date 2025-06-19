/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SVC_EX_exception {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SVC_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl SVC_EX_exception {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100000u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b00001u32 << 0u32,
            )
        }
    }
}
pub mod HVC_EX_exception {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct HVC_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl HVC_EX_exception {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100000u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b00010u32 << 0u32,
            )
        }
    }
}
pub mod SMC_EX_exception {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMC_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl SMC_EX_exception {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100000u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b00011u32 << 0u32,
            )
        }
    }
}
pub mod BRK_EX_exception {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BRK_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl BRK_EX_exception {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100001u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b00000u32 << 0u32,
            )
        }
    }
}
pub mod HLT_EX_exception {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct HLT_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl HLT_EX_exception {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100010u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b00000u32 << 0u32,
            )
        }
    }
}
pub mod TCANCEL_EX_exception {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TCANCEL_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl TCANCEL_EX_exception {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100011u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b00000u32 << 0u32,
            )
        }
    }
}
pub mod DCPS1_DC_exception {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DCPS1_DC_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl DCPS1_DC_exception {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100101u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b00001u32 << 0u32,
            )
        }
    }
}
pub mod DCPS2_DC_exception {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DCPS2_DC_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl DCPS2_DC_exception {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100101u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b00010u32 << 0u32,
            )
        }
    }
}
pub mod DCPS3_DC_exception {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DCPS3_DC_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl DCPS3_DC_exception {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100101u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b00011u32 << 0u32,
            )
        }
    }
}
