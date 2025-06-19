/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smop4a_za32_zz_h1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4a_za32_zz_h1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4a_za32_zz_h1x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0010u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod umop4a_za32_zz_h1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4a_za32_zz_h1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4a_za32_zz_h1x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0010u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod smop4s_za32_zz_h1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4s_za32_zz_h1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4s_za32_zz_h1x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0110u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod umop4s_za32_zz_h1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4s_za32_zz_h1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4s_za32_zz_h1x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0110u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod smop4a_za32_zz_h1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4a_za32_zz_h1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4a_za32_zz_h1x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0010u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod umop4a_za32_zz_h1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4a_za32_zz_h1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4a_za32_zz_h1x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0010u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod smop4s_za32_zz_h1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4s_za32_zz_h1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4s_za32_zz_h1x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0110u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod umop4s_za32_zz_h1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4s_za32_zz_h1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4s_za32_zz_h1x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0110u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod smop4a_za32_zz_h2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4a_za32_zz_h2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4a_za32_zz_h2x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0010u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod umop4a_za32_zz_h2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4a_za32_zz_h2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4a_za32_zz_h2x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0010u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod smop4s_za32_zz_h2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4s_za32_zz_h2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4s_za32_zz_h2x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0110u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod umop4s_za32_zz_h2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4s_za32_zz_h2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4s_za32_zz_h2x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0110u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod smop4a_za32_zz_h2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4a_za32_zz_h2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4a_za32_zz_h2x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0010u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod umop4a_za32_zz_h2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4a_za32_zz_h2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4a_za32_zz_h2x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0010u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod smop4s_za32_zz_h2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smop4s_za32_zz_h2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl smop4s_za32_zz_h2x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0110u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod umop4s_za32_zz_h2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umop4s_za32_zz_h2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl umop4s_za32_zz_h2x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001000u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0100000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0110u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
