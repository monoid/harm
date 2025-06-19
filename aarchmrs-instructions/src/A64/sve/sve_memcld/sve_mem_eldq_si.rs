/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld2q_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld2q_z_p_bi_contiguous {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld2q_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101001001001u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld3q_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld3q_z_p_bi_contiguous {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld3q_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101001010001u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld4q_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld4q_z_p_bi_contiguous {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld4q_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101001011001u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
