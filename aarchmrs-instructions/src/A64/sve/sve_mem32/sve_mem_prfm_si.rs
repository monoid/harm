/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod prfb_i_p_bi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfb_i_p_bi_s {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfb_i_p_bi_s {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm6,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010111u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b000u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
pub mod prfh_i_p_bi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfh_i_p_bi_s {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfh_i_p_bi_s {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm6,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010111u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
pub mod prfw_i_p_bi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfw_i_p_bi_s {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfw_i_p_bi_s {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm6,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010111u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b010u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
pub mod prfd_i_p_bi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfd_i_p_bi_s {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfd_i_p_bi_s {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm6,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010111u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b011u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
