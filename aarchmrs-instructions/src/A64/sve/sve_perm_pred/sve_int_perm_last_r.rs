/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod lasta_r_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lasta_r_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl lasta_r_p_z_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            B: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                B: B.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10000u32 << 17u32
                    | u32::from(self.B) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod lastb_r_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lastb_r_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl lastb_r_p_z_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            B: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                B: B.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10000u32 << 17u32
                    | u32::from(self.B) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
