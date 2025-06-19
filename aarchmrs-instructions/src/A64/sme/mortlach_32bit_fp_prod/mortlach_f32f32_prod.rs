/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmopa_za_pp_zz_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmopa_za_pp_zz_32 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl fmopa_za_pp_zz_32 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pm: impl Into<::aarchmrs_types::BitValue<3>>,
            Pn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Pm: Pm.into(),
                Pn: Pn.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000100u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod fmops_za_pp_zz_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmops_za_pp_zz_32 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl fmops_za_pp_zz_32 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pm: impl Into<::aarchmrs_types::BitValue<3>>,
            Pn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Pm: Pm.into(),
                Pn: Pn.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000100u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
