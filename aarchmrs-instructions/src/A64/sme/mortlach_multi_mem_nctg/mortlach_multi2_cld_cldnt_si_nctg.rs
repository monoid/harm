/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_mzx_p_bi_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_mzx_p_bi_2x8 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1b_mzx_p_bi_2x8 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                imm4,
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
                0b101000010100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
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
pub mod ldnt1b_mzx_p_bi_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1b_mzx_p_bi_2x8 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1b_mzx_p_bi_2x8 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                imm4,
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
                0b101000010100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
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
pub mod ld1h_mzx_p_bi_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_mzx_p_bi_2x8 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1h_mzx_p_bi_2x8 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                imm4,
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
                0b101000010100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
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
pub mod ldnt1h_mzx_p_bi_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1h_mzx_p_bi_2x8 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1h_mzx_p_bi_2x8 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                imm4,
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
                0b101000010100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
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
pub mod ld1w_mzx_p_bi_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_mzx_p_bi_2x8 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1w_mzx_p_bi_2x8 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                imm4,
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
                0b101000010100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
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
pub mod ldnt1w_mzx_p_bi_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1w_mzx_p_bi_2x8 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1w_mzx_p_bi_2x8 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                imm4,
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
                0b101000010100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
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
pub mod ld1d_mzx_p_bi_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_mzx_p_bi_2x8 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1d_mzx_p_bi_2x8 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                imm4,
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
                0b101000010100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
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
pub mod ldnt1d_mzx_p_bi_2x8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1d_mzx_p_bi_2x8 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1d_mzx_p_bi_2x8 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            T: ::aarchmrs_types::BitValue<1>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                imm4,
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
                0b101000010100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
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
