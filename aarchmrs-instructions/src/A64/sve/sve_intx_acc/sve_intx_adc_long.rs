/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod adclb_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct adclb_z_zzz_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl adclb_z_zzz_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zm: Zm.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b11010u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod sbclb_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sbclb_z_zzz_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl sbclb_z_zzz_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zm: Zm.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001011u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b11010u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod adclt_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct adclt_z_zzz_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl adclt_z_zzz_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zm: Zm.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b11010u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod sbclt_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sbclt_z_zzz_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl sbclt_z_zzz_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zm: Zm.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001011u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b11010u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
