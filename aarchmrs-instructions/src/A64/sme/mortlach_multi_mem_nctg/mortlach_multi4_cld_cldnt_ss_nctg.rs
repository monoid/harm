/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_mzx_p_br_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_mzx_p_br_4x4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<2>,
    }
    impl ld1b_mzx_p_br_4x4 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zt: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                T: T.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.T) << 4u32
                    | 0b00u32 << 2u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnt1b_mzx_p_br_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1b_mzx_p_br_4x4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<2>,
    }
    impl ldnt1b_mzx_p_br_4x4 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zt: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                T: T.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.T) << 4u32
                    | 0b10u32 << 2u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1h_mzx_p_br_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_mzx_p_br_4x4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<2>,
    }
    impl ld1h_mzx_p_br_4x4 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zt: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                T: T.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.T) << 4u32
                    | 0b00u32 << 2u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnt1h_mzx_p_br_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1h_mzx_p_br_4x4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<2>,
    }
    impl ldnt1h_mzx_p_br_4x4 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zt: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                T: T.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.T) << 4u32
                    | 0b10u32 << 2u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1w_mzx_p_br_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_mzx_p_br_4x4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<2>,
    }
    impl ld1w_mzx_p_br_4x4 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zt: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                T: T.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.T) << 4u32
                    | 0b00u32 << 2u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnt1w_mzx_p_br_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1w_mzx_p_br_4x4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<2>,
    }
    impl ldnt1w_mzx_p_br_4x4 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zt: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                T: T.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.T) << 4u32
                    | 0b10u32 << 2u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1d_mzx_p_br_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_mzx_p_br_4x4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<2>,
    }
    impl ld1d_mzx_p_br_4x4 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zt: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                T: T.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.T) << 4u32
                    | 0b00u32 << 2u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnt1d_mzx_p_br_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1d_mzx_p_br_4x4 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zt: ::aarchmrs_types::BitValue<2>,
    }
    impl ldnt1d_mzx_p_br_4x4 {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zt: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                T: T.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10100001000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.T) << 4u32
                    | 0b10u32 << 2u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
