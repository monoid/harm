/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_mz_p_bi_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl ld1b_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1b_mz_p_bi_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1b_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl ldnt1b_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
pub mod ld1h_mz_p_bi_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl ld1h_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1h_mz_p_bi_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1h_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl ldnt1h_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
pub mod ld1w_mz_p_bi_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl ld1w_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1w_mz_p_bi_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1w_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl ldnt1w_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
pub mod ld1d_mz_p_bi_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl ld1d_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1d_mz_p_bi_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1d_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl ldnt1d_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b1u32 << 0u32,
            )
        }
    }
}
