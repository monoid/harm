/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmop4a_za_zz_d1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za_zz_d1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl fmop4a_za_zz_d1x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000110u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b001u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmop4s_za_zz_d1x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4s_za_zz_d1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl fmop4s_za_zz_d1x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000110u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b011u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmop4a_za_zz_d1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za_zz_d1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl fmop4a_za_zz_d1x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000110u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b001u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmop4s_za_zz_d1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4s_za_zz_d1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl fmop4s_za_zz_d1x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000110u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b011u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmop4a_za_zz_d2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za_zz_d2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl fmop4a_za_zz_d2x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000110u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b001u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmop4s_za_zz_d2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4s_za_zz_d2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl fmop4s_za_zz_d2x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000110u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b011u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmop4a_za_zz_d2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za_zz_d2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl fmop4a_za_zz_d2x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000110u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b001u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmop4s_za_zz_d2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4s_za_zz_d2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl fmop4s_za_zz_d2x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000110u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b011u32 << 3u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
