/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod f1cvt_mz2_z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct f1cvt_mz2_z8_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
        pub L: ::aarchmrs_types::BitValue<1>,
    }
    impl f1cvt_mz2_z8_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<4>,
            L: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { Zn, Zd, L }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100100110111000u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | self.L.into_inner() << 0u32,
            )
        }
    }
}
pub mod bf1cvt_mz2_z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bf1cvt_mz2_z8_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
        pub L: ::aarchmrs_types::BitValue<1>,
    }
    impl bf1cvt_mz2_z8_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<4>,
            L: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { Zn, Zd, L }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000101100110111000u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | self.L.into_inner() << 0u32,
            )
        }
    }
}
pub mod f2cvt_mz2_z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct f2cvt_mz2_z8_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
        pub L: ::aarchmrs_types::BitValue<1>,
    }
    impl f2cvt_mz2_z8_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<4>,
            L: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { Zn, Zd, L }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000110100110111000u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | self.L.into_inner() << 0u32,
            )
        }
    }
}
pub mod bf2cvt_mz2_z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bf2cvt_mz2_z8_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
        pub L: ::aarchmrs_types::BitValue<1>,
    }
    impl bf2cvt_mz2_z8_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<4>,
            L: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { Zn, Zd, L }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000111100110111000u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | self.L.into_inner() << 0u32,
            )
        }
    }
}
pub mod f1cvtl_mz2_z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct f1cvtl_mz2_z8_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
        pub L: ::aarchmrs_types::BitValue<1>,
    }
    impl f1cvtl_mz2_z8_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<4>,
            L: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { Zn, Zd, L }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100100110111000u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | self.L.into_inner() << 0u32,
            )
        }
    }
}
pub mod bf1cvtl_mz2_z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bf1cvtl_mz2_z8_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
        pub L: ::aarchmrs_types::BitValue<1>,
    }
    impl bf1cvtl_mz2_z8_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<4>,
            L: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { Zn, Zd, L }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000101100110111000u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | self.L.into_inner() << 0u32,
            )
        }
    }
}
pub mod f2cvtl_mz2_z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct f2cvtl_mz2_z8_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
        pub L: ::aarchmrs_types::BitValue<1>,
    }
    impl f2cvtl_mz2_z8_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<4>,
            L: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { Zn, Zd, L }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000110100110111000u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | self.L.into_inner() << 0u32,
            )
        }
    }
}
pub mod bf2cvtl_mz2_z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bf2cvtl_mz2_z8_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
        pub L: ::aarchmrs_types::BitValue<1>,
    }
    impl bf2cvtl_mz2_z8_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<4>,
            L: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { Zn, Zd, L }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000111100110111000u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | self.L.into_inner() << 0u32,
            )
        }
    }
}
