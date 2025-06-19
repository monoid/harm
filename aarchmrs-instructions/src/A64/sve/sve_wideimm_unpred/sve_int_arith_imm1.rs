/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smax_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smax_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl smax_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10100u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod smin_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smin_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl smin_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10101u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod umax_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umax_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl umax_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10100u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod umin_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umin_z_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl umin_z_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                imm8: imm8.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10101u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
