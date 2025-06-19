/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlall_za_zzv_2x1 {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl smlall_za_zzv_2x1 {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b00u32 << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
pub mod usmlall_za_zzv_s2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmlall_za_zzv_s2x1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl usmlall_za_zzv_s2x1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                U: U.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b010u32 << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
pub mod smlsll_za_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlsll_za_zzv_2x1 {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl smlsll_za_zzv_2x1 {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b00u32 << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
pub mod umlall_za_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlall_za_zzv_2x1 {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl umlall_za_zzv_2x1 {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b00u32 << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
pub mod sumlall_za_zzv_s2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumlall_za_zzv_s2x1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl sumlall_za_zzv_s2x1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                U: U.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b010u32 << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
pub mod umlsll_za_zzv_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlsll_za_zzv_2x1 {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl umlsll_za_zzv_2x1 {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b00u32 << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
