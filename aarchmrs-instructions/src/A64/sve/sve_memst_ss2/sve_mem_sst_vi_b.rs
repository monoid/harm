/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_z_p_ai_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1b_z_p_ai_s {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1b_z_p_ai_s {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100100011u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st1h_z_p_ai_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1h_z_p_ai_s {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1h_z_p_ai_s {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100100111u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st1w_z_p_ai_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1w_z_p_ai_s {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1w_z_p_ai_s {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100101011u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
