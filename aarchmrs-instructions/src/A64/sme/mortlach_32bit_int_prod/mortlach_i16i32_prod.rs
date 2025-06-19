/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smopa_za32_pp_zz_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smopa_za32_pp_zz_16 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smopa_za32_pp_zz_16 {
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
                0b10100000100u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b010u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod umopa_za32_pp_zz_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umopa_za32_pp_zz_16 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umopa_za32_pp_zz_16 {
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
                0b10100001100u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b010u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod smops_za32_pp_zz_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smops_za32_pp_zz_16 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smops_za32_pp_zz_16 {
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
                0b10100000100u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b110u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod umops_za32_pp_zz_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umops_za32_pp_zz_16 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umops_za32_pp_zz_16 {
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
                0b10100001100u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b110u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
