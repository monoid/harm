/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod prfb_i_p_ai_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfb_i_p_ai_s {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfb_i_p_ai_s {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm5,
                Pg,
                Zn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000100000u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b111u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
pub mod prfh_i_p_ai_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfh_i_p_ai_s {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfh_i_p_ai_s {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm5,
                Pg,
                Zn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000100100u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b111u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
pub mod prfw_i_p_ai_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfw_i_p_ai_s {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfw_i_p_ai_s {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm5,
                Pg,
                Zn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000101000u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b111u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
pub mod prfd_i_p_ai_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfd_i_p_ai_s {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfd_i_p_ai_s {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm5,
                Pg,
                Zn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000101100u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b111u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
