/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilewr_p_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilewr_p_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl whilewr_p_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { size, Rm, Rn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001100u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod whilerw_p_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilerw_p_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl whilerw_p_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { size, Rm, Rn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001100u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
