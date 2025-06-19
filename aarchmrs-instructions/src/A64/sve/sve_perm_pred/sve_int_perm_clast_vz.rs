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
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Vdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zm, Vdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b101010100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Vdn.into_inner() << 0u32,
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
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Vdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zm, Vdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b101011100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Vdn.into_inner() << 0u32,
            )
        }
    }
}
