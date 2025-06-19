/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod add_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct add_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl add_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            sh: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                sh: sh.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10000011u32 << 14u32
                    | u32::from(self.sh) << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sub_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sub_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sub_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            sh: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                sh: sh.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10000111u32 << 14u32
                    | u32::from(self.sh) << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod subr_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct subr_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl subr_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            sh: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                sh: sh.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10001111u32 << 14u32
                    | u32::from(self.sh) << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sqadd_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqadd_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqadd_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            sh: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                sh: sh.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10010u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.sh) << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sqsub_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqsub_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqsub_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            sh: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                sh: sh.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10011u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.sh) << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod uqadd_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqadd_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqadd_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            sh: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                sh: sh.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10010u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.sh) << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod uqsub_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqsub_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqsub_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            sh: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                sh: sh.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10011u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.sh) << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
