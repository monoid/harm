/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod asr_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct asr_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl asr_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                U: U.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b10010u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod lsl_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lsl_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl lsl_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b100111u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod lsr_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lsr_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl lsr_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                U: U.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b10010u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
