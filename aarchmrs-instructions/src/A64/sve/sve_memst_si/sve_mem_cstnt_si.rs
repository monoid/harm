/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod stnt1b_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1b_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl stnt1b_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b001u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod stnt1h_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1h_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl stnt1h_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b001u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod stnt1w_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1w_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl stnt1w_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b001u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod stnt1d_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1d_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl stnt1d_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b001u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
