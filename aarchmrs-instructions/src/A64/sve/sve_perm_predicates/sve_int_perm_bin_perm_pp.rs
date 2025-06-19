/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zip1_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zip1_p_pp_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl zip1_p_pp_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                Pm,
                H,
                Pn,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01000u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod uzp1_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uzp1_p_pp_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl uzp1_p_pp_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                Pm,
                H,
                Pn,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01001u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod trn1_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct trn1_p_pp_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl trn1_p_pp_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                Pm,
                H,
                Pn,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01010u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod zip2_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zip2_p_pp_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl zip2_p_pp_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                Pm,
                H,
                Pn,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01000u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod uzp2_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uzp2_p_pp_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl uzp2_p_pp_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                Pm,
                H,
                Pn,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01001u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod trn2_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct trn2_p_pp_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl trn2_p_pp_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                Pm,
                H,
                Pn,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01010u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
