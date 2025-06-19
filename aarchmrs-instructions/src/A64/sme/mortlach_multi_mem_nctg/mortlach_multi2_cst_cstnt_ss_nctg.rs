/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_mzx_p_br_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1b_mzx_p_br_2x8 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl st1b_mzx_p_br_2x8 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                T,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.T.into_inner() << 4u32
                    | 0b0u32 << 3u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod stnt1b_mzx_p_br_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1b_mzx_p_br_2x8 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl stnt1b_mzx_p_br_2x8 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                T,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.T.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod st1h_mzx_p_br_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1h_mzx_p_br_2x8 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl st1h_mzx_p_br_2x8 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                T,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.T.into_inner() << 4u32
                    | 0b0u32 << 3u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod stnt1h_mzx_p_br_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1h_mzx_p_br_2x8 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl stnt1h_mzx_p_br_2x8 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                T,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.T.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod st1w_mzx_p_br_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1w_mzx_p_br_2x8 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl st1w_mzx_p_br_2x8 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                T,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.T.into_inner() << 4u32
                    | 0b0u32 << 3u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod stnt1w_mzx_p_br_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1w_mzx_p_br_2x8 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl stnt1w_mzx_p_br_2x8 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                T,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.T.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod st1d_mzx_p_br_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1d_mzx_p_br_2x8 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl st1d_mzx_p_br_2x8 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                T,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.T.into_inner() << 4u32
                    | 0b0u32 << 3u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod stnt1d_mzx_p_br_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1d_mzx_p_br_2x8 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl stnt1d_mzx_p_br_2x8 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                T,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.T.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
