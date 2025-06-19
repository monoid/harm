/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod xar_z_zzi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct xar_z_zzi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl xar_z_zzi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
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
                    | 0b001101u32 << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
