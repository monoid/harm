/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smopa_za_pp_zz_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smopa_za_pp_zz_64 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl smopa_za_pp_zz_64 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Pm,
                Pn,
                Zn,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000110u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b00u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod sumopa_za_pp_zz_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumopa_za_pp_zz_64 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl sumopa_za_pp_zz_64 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Pm,
                Pn,
                Zn,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b00u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod usmopa_za_pp_zz_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmopa_za_pp_zz_64 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl usmopa_za_pp_zz_64 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Pm,
                Pn,
                Zn,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001110u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b00u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod umopa_za_pp_zz_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umopa_za_pp_zz_64 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl umopa_za_pp_zz_64 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Pm,
                Pn,
                Zn,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b00u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod smops_za_pp_zz_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smops_za_pp_zz_64 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl smops_za_pp_zz_64 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Pm,
                Pn,
                Zn,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000110u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b10u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod sumops_za_pp_zz_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sumops_za_pp_zz_64 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl sumops_za_pp_zz_64 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Pm,
                Pn,
                Zn,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b10u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod usmops_za_pp_zz_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usmops_za_pp_zz_64 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl usmops_za_pp_zz_64 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Pm,
                Pn,
                Zn,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001110u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b10u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod umops_za_pp_zz_64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umops_za_pp_zz_64 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl umops_za_pp_zz_64 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Zm,
                Pm,
                Pn,
                Zn,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b10u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
