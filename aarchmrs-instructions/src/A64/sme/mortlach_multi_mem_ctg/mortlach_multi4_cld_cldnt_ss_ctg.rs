/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_mz_p_br_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_mz_p_br_4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1b_mz_p_br_4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1b_mz_p_br_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1b_mz_p_br_4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1b_mz_p_br_4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b01u32 << 0u32,
            )
        }
    }
}
pub mod ld1h_mz_p_br_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_mz_p_br_4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1h_mz_p_br_4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1h_mz_p_br_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1h_mz_p_br_4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1h_mz_p_br_4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b01u32 << 0u32,
            )
        }
    }
}
pub mod ld1w_mz_p_br_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_mz_p_br_4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1w_mz_p_br_4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1w_mz_p_br_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1w_mz_p_br_4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1w_mz_p_br_4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b01u32 << 0u32,
            )
        }
    }
}
pub mod ld1d_mz_p_br_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_mz_p_br_4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1d_mz_p_br_4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1d_mz_p_br_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1d_mz_p_br_4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1d_mz_p_br_4 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100000000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b01u32 << 0u32,
            )
        }
    }
}
