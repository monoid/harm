/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smop4a_za_zz_b1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4a_za_zz_b1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4a_za_zz_b1x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod sumop4a_za_zz_b1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumop4a_za_zz_b1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl sumop4a_za_zz_b1x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod usmop4a_za_zz_b1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmop4a_za_zz_b1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl usmop4a_za_zz_b1x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod umop4a_za_zz_b1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4a_za_zz_b1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4a_za_zz_b1x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000001001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod smop4s_za_zz_b1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4s_za_zz_b1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4s_za_zz_b1x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod sumop4s_za_zz_b1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumop4s_za_zz_b1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl sumop4s_za_zz_b1x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod usmop4s_za_zz_b1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmop4s_za_zz_b1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl usmop4s_za_zz_b1x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod umop4s_za_zz_b1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4s_za_zz_b1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4s_za_zz_b1x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000001001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod smop4a_za_zz_b1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4a_za_zz_b1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4a_za_zz_b1x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod sumop4a_za_zz_b1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumop4a_za_zz_b1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl sumop4a_za_zz_b1x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod usmop4a_za_zz_b1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmop4a_za_zz_b1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl usmop4a_za_zz_b1x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod umop4a_za_zz_b1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4a_za_zz_b1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4a_za_zz_b1x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000001001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod smop4s_za_zz_b1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4s_za_zz_b1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4s_za_zz_b1x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod sumop4s_za_zz_b1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumop4s_za_zz_b1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl sumop4s_za_zz_b1x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod usmop4s_za_zz_b1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmop4s_za_zz_b1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl usmop4s_za_zz_b1x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod umop4s_za_zz_b1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4s_za_zz_b1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4s_za_zz_b1x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000001001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod smop4a_za_zz_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4a_za_zz_b2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4a_za_zz_b2x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod sumop4a_za_zz_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumop4a_za_zz_b2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl sumop4a_za_zz_b2x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod usmop4a_za_zz_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmop4a_za_zz_b2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl usmop4a_za_zz_b2x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod umop4a_za_zz_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4a_za_zz_b2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4a_za_zz_b2x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000001001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod smop4s_za_zz_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4s_za_zz_b2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4s_za_zz_b2x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod sumop4s_za_zz_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumop4s_za_zz_b2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl sumop4s_za_zz_b2x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod usmop4s_za_zz_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmop4s_za_zz_b2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl usmop4s_za_zz_b2x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod umop4s_za_zz_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4s_za_zz_b2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4s_za_zz_b2x1 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000001001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod smop4a_za_zz_b2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4a_za_zz_b2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4a_za_zz_b2x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod sumop4a_za_zz_b2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumop4a_za_zz_b2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl sumop4a_za_zz_b2x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod usmop4a_za_zz_b2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmop4a_za_zz_b2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl usmop4a_za_zz_b2x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod umop4a_za_zz_b2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4a_za_zz_b2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4a_za_zz_b2x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000001001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0000u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod smop4s_za_zz_b2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4s_za_zz_b2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4s_za_zz_b2x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000000u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod sumop4s_za_zz_b2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumop4s_za_zz_b2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl sumop4s_za_zz_b2x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000000001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod usmop4s_za_zz_b2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmop4s_za_zz_b2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl usmop4s_za_zz_b2x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
pub mod umop4s_za_zz_b2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4s_za_zz_b2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4s_za_zz_b2x2 {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            N: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
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
                0b10000001001u32 << 21u32
                    | u32::from(self.M) << 20u32
                    | u32::from(self.Zm) << 17u32
                    | 0b0100000u32 << 10u32
                    | u32::from(self.N) << 9u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0100u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
