/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1b_mz_p_bi_4 {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1b_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1b_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1b_mz_p_bi_4 {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 2u32
                    | 0b01u32 << 0u32,
            )
        }
    }
}
pub mod ld1h_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1h_mz_p_bi_4 {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1h_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1h_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1h_mz_p_bi_4 {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 2u32
                    | 0b01u32 << 0u32,
            )
        }
    }
}
pub mod ld1w_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1w_mz_p_bi_4 {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1w_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1w_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1w_mz_p_bi_4 {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 2u32
                    | 0b01u32 << 0u32,
            )
        }
    }
}
pub mod ld1d_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1d_mz_p_bi_4 {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod ldnt1d_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1d_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1d_mz_p_bi_4 {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            PNg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                msz: msz.into(),
                PNg: PNg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000100u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.PNg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 2u32
                    | 0b01u32 << 0u32,
            )
        }
    }
}
