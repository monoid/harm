/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod prfb_i_p_bz_s_x32_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfb_i_p_bz_s_x32_scaled {
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfb_i_p_bz_s_x32_scaled {
        #[inline]
        pub fn new(
            xs: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                xs: xs.into(),
                Zm: Zm.into(),
                msz: msz.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100001000u32 << 23u32
                    | u32::from(self.xs) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
pub mod prfh_i_p_bz_s_x32_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfh_i_p_bz_s_x32_scaled {
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfh_i_p_bz_s_x32_scaled {
        #[inline]
        pub fn new(
            xs: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                xs: xs.into(),
                Zm: Zm.into(),
                msz: msz.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100001000u32 << 23u32
                    | u32::from(self.xs) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
pub mod prfw_i_p_bz_s_x32_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfw_i_p_bz_s_x32_scaled {
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfw_i_p_bz_s_x32_scaled {
        #[inline]
        pub fn new(
            xs: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                xs: xs.into(),
                Zm: Zm.into(),
                msz: msz.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100001000u32 << 23u32
                    | u32::from(self.xs) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
pub mod prfd_i_p_bz_s_x32_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfd_i_p_bz_s_x32_scaled {
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfd_i_p_bz_s_x32_scaled {
        #[inline]
        pub fn new(
            xs: impl Into<::aarchmrs_types::BitValue<1>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            prfop: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                xs: xs.into(),
                Zm: Zm.into(),
                msz: msz.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                prfop: prfop.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100001000u32 << 23u32
                    | u32::from(self.xs) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.msz) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.prfop) << 0u32,
            )
        }
    }
}
