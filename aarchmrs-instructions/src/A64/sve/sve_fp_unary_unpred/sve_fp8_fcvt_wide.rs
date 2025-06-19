/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod f1cvt_z_z8_b2h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct f1cvt_z_z8_b2h {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl f1cvt_z_z8_b2h {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { L, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001010000100u32 << 17u32
                    | self.L.into_inner() << 16u32
                    | 0b001100u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod f2cvt_z_z8_b2h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct f2cvt_z_z8_b2h {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl f2cvt_z_z8_b2h {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { L, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001010000100u32 << 17u32
                    | self.L.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod bf1cvt_z_z8_b2bf {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bf1cvt_z_z8_b2bf {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl bf1cvt_z_z8_b2bf {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { L, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001010000100u32 << 17u32
                    | self.L.into_inner() << 16u32
                    | 0b001110u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod bf2cvt_z_z8_b2bf {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bf2cvt_z_z8_b2bf {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl bf2cvt_z_z8_b2bf {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { L, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001010000100u32 << 17u32
                    | self.L.into_inner() << 16u32
                    | 0b001111u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod f1cvtlt_z_z8_b2h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct f1cvtlt_z_z8_b2h {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl f1cvtlt_z_z8_b2h {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { L, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001010000100u32 << 17u32
                    | self.L.into_inner() << 16u32
                    | 0b001100u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod f2cvtlt_z_z8_b2h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct f2cvtlt_z_z8_b2h {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl f2cvtlt_z_z8_b2h {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { L, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001010000100u32 << 17u32
                    | self.L.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod bf1cvtlt_z_z8_b2bf {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bf1cvtlt_z_z8_b2bf {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl bf1cvtlt_z_z8_b2bf {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { L, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001010000100u32 << 17u32
                    | self.L.into_inner() << 16u32
                    | 0b001110u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod bf2cvtlt_z_z8_b2bf {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bf2cvtlt_z_z8_b2bf {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl bf2cvtlt_z_z8_b2bf {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { L, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011001010000100u32 << 17u32
                    | self.L.into_inner() << 16u32
                    | 0b001111u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
