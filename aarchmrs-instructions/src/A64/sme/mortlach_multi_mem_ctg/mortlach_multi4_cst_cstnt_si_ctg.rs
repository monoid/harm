/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1b_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl st1b_mz_p_bi_4 {
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
                0b101000000110u32 << 20u32
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
pub mod stnt1b_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1b_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl stnt1b_mz_p_bi_4 {
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
                0b101000000110u32 << 20u32
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
pub mod st1h_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1h_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl st1h_mz_p_bi_4 {
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
                0b101000000110u32 << 20u32
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
pub mod stnt1h_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1h_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl stnt1h_mz_p_bi_4 {
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
                0b101000000110u32 << 20u32
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
pub mod st1w_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1w_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl st1w_mz_p_bi_4 {
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
                0b101000000110u32 << 20u32
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
pub mod stnt1w_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1w_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl stnt1w_mz_p_bi_4 {
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
                0b101000000110u32 << 20u32
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
pub mod st1d_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1d_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl st1d_mz_p_bi_4 {
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
                0b101000000110u32 << 20u32
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
pub mod stnt1d_mz_p_bi_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1d_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl stnt1d_mz_p_bi_4 {
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
                0b101000000110u32 << 20u32
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
