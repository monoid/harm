/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod clasta_v_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct clasta_v_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Vdn: ::aarchmrs_types::BitValue<5>,
    }
    impl clasta_v_p_z_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Vdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Vdn: Vdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b101010100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Vdn) << 0u32,
            )
        }
    }
}
pub mod clastb_v_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct clastb_v_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Vdn: ::aarchmrs_types::BitValue<5>,
    }
    impl clastb_v_p_z_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Vdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Vdn: Vdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b101011100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Vdn) << 0u32,
            )
        }
    }
}
