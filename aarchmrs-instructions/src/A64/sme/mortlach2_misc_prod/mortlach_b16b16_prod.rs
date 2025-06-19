/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfmopa_za_pp_zz_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmopa_za_pp_zz_16 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl bfmopa_za_pp_zz_16 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pm: impl Into<::aarchmrs_types::BitValue<3>>,
            Pn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
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
                0b10000001101u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0100u32 << 1u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod bfmops_za_pp_zz_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmops_za_pp_zz_16 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl bfmops_za_pp_zz_16 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pm: impl Into<::aarchmrs_types::BitValue<3>>,
            Pn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
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
                0b10000001101u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b1100u32 << 1u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
