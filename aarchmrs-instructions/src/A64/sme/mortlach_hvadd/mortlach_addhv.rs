/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod addha_za_pp_z_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addha_za_pp_z_32 {
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl addha_za_pp_z_32 {
        #[inline]
        pub fn new(
            Pm: impl Into<::aarchmrs_types::BitValue<3>>,
            Pn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Pm: Pm.into(),
                Pn: Pn.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000010010000u32 << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod addha_za_pp_z_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addha_za_pp_z_64 {
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl addha_za_pp_z_64 {
        #[inline]
        pub fn new(
            Pm: impl Into<::aarchmrs_types::BitValue<3>>,
            Pn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Pm: Pm.into(),
                Pn: Pn.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011010000u32 << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b00u32 << 3u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod addva_za_pp_z_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addva_za_pp_z_32 {
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl addva_za_pp_z_32 {
        #[inline]
        pub fn new(
            Pm: impl Into<::aarchmrs_types::BitValue<3>>,
            Pn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Pm: Pm.into(),
                Pn: Pn.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000010010001u32 << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod addva_za_pp_z_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addva_za_pp_z_64 {
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl addva_za_pp_z_64 {
        #[inline]
        pub fn new(
            Pm: impl Into<::aarchmrs_types::BitValue<3>>,
            Pn: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Pm: Pm.into(),
                Pn: Pn.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011010001u32 << 16u32
                    | u32::from(self.Pm) << 13u32
                    | u32::from(self.Pn) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b00u32 << 3u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
