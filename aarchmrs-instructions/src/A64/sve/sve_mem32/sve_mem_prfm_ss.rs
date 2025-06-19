/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod prfb_i_p_br_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfb_i_p_br_s {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfb_i_p_br_s {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000100000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
pub mod prfh_i_p_br_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfh_i_p_br_s {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfh_i_p_br_s {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000100100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
pub mod prfw_i_p_br_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfw_i_p_br_s {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfw_i_p_br_s {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000101000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
pub mod prfd_i_p_br_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfd_i_p_br_s {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfd_i_p_br_s {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000101100u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
