/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incp_r_p_r_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct incp_r_p_r_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl incp_r_p_r_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pm, Rdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1011001000100u32 << 9u32
                    | self.Pm.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod decp_r_p_r_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct decp_r_p_r_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl decp_r_p_r_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pm, Rdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1011011000100u32 << 9u32
                    | self.Pm.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
