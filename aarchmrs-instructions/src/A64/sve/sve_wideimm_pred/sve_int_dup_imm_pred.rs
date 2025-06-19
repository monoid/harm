/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cpy_z_o_i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cpy_z_o_i_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl cpy_z_o_i_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            sh: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                sh: sh.into(),
                imm8: imm8.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01u32 << 20u32
                    | u32::from(self.Pg) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.sh) << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod cpy_z_p_i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cpy_z_p_i_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl cpy_z_p_i_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            sh: impl Into<::aarchmrs_types::BitValue<1>>,
            imm8: impl Into<::aarchmrs_types::BitValue<8>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                sh: sh.into(),
                imm8: imm8.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01u32 << 20u32
                    | u32::from(self.Pg) << 16u32
                    | 0b01u32 << 14u32
                    | u32::from(self.sh) << 13u32
                    | u32::from(self.imm8) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
