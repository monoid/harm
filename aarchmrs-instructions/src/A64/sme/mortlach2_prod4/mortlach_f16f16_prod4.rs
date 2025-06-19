/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmop4a_za_zz_h1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za_zz_h1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4a_za_zz_h1x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                M: M.into(),
                Zm: Zm.into(),
                N: N.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0000000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b00100u32 << 1u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod fmop4s_za_zz_h1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4s_za_zz_h1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4s_za_zz_h1x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                M: M.into(),
                Zm: Zm.into(),
                N: N.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0000000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b01100u32 << 1u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod fmop4a_za_zz_h1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za_zz_h1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4a_za_zz_h1x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                M: M.into(),
                Zm: Zm.into(),
                N: N.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0000000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b00100u32 << 1u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod fmop4s_za_zz_h1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4s_za_zz_h1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4s_za_zz_h1x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                M: M.into(),
                Zm: Zm.into(),
                N: N.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0000000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b01100u32 << 1u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod fmop4a_za_zz_h2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za_zz_h2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4a_za_zz_h2x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                M: M.into(),
                Zm: Zm.into(),
                N: N.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0000000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b00100u32 << 1u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod fmop4s_za_zz_h2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4s_za_zz_h2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4s_za_zz_h2x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                M: M.into(),
                Zm: Zm.into(),
                N: N.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0000000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b01100u32 << 1u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod fmop4a_za_zz_h2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za_zz_h2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4a_za_zz_h2x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                M: M.into(),
                Zm: Zm.into(),
                N: N.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0000000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b00100u32 << 1u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod fmop4s_za_zz_h2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4s_za_zz_h2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4s_za_zz_h2x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                M: M.into(),
                Zm: Zm.into(),
                N: N.into(),
                Zn: Zn.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0000000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b01100u32 << 1u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
