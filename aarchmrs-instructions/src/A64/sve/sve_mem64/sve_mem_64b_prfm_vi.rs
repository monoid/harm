/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod prfb_i_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfb_i_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfb_i_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100000u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
pub mod prfh_i_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfh_i_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfh_i_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100100u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
pub mod prfw_i_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfw_i_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfw_i_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101000u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
pub mod prfd_i_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfd_i_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfd_i_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101100u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
