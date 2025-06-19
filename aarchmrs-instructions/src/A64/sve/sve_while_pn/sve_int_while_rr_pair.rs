/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilege_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilege_pp_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<3>,
            eq: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self {
                size,
                Rm,
                lt,
                Rn,
                Pd,
                eq,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01010u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 1u32
                    | self.eq.into_inner() << 0u32,
            )
        }
    }
}
pub mod whilehs_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilehs_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilehs_pp_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<3>,
            eq: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self {
                size,
                Rm,
                lt,
                Rn,
                Pd,
                eq,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01011u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 1u32
                    | self.eq.into_inner() << 0u32,
            )
        }
    }
}
pub mod whilegt_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilegt_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilegt_pp_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<3>,
            eq: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self {
                size,
                Rm,
                lt,
                Rn,
                Pd,
                eq,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01010u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 1u32
                    | self.eq.into_inner() << 0u32,
            )
        }
    }
}
pub mod whilehi_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilehi_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilehi_pp_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<3>,
            eq: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self {
                size,
                Rm,
                lt,
                Rn,
                Pd,
                eq,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01011u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 1u32
                    | self.eq.into_inner() << 0u32,
            )
        }
    }
}
pub mod whilelt_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilelt_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilelt_pp_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<3>,
            eq: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self {
                size,
                Rm,
                lt,
                Rn,
                Pd,
                eq,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01010u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 1u32
                    | self.eq.into_inner() << 0u32,
            )
        }
    }
}
pub mod whilelo_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilelo_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilelo_pp_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<3>,
            eq: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self {
                size,
                Rm,
                lt,
                Rn,
                Pd,
                eq,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01011u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 1u32
                    | self.eq.into_inner() << 0u32,
            )
        }
    }
}
pub mod whilele_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilele_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilele_pp_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<3>,
            eq: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self {
                size,
                Rm,
                lt,
                Rn,
                Pd,
                eq,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01010u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 1u32
                    | self.eq.into_inner() << 0u32,
            )
        }
    }
}
pub mod whilels_pp_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilels_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<3>,
        pub eq: ::aarchmrs_types::BitValue<1>,
    }
    impl whilels_pp_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<3>,
            eq: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self {
                size,
                Rm,
                lt,
                Rn,
                Pd,
                eq,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01011u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 1u32
                    | self.eq.into_inner() << 0u32,
            )
        }
    }
}
