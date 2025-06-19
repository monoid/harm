/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmphs_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmphs_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmphs_p_p_zi_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm7: ::aarchmrs_types::BitValue<7>,
            lt: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ne: ::aarchmrs_types::BitValue<1>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                imm7,
                lt,
                Pg,
                Zn,
                ne,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm7.into_inner() << 14u32
                    | self.lt.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.ne.into_inner() << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod cmphi_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmphi_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmphi_p_p_zi_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm7: ::aarchmrs_types::BitValue<7>,
            lt: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ne: ::aarchmrs_types::BitValue<1>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                imm7,
                lt,
                Pg,
                Zn,
                ne,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm7.into_inner() << 14u32
                    | self.lt.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.ne.into_inner() << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod cmplo_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmplo_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmplo_p_p_zi_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm7: ::aarchmrs_types::BitValue<7>,
            lt: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ne: ::aarchmrs_types::BitValue<1>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                imm7,
                lt,
                Pg,
                Zn,
                ne,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm7.into_inner() << 14u32
                    | self.lt.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.ne.into_inner() << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod cmpls_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmpls_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmpls_p_p_zi_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm7: ::aarchmrs_types::BitValue<7>,
            lt: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ne: ::aarchmrs_types::BitValue<1>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                imm7,
                lt,
                Pg,
                Zn,
                ne,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm7.into_inner() << 14u32
                    | self.lt.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.ne.into_inner() << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
