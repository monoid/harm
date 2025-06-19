/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zip1_z_zz_q {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zip1_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl zip1_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00000u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod uzp1_z_zz_q {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uzp1_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uzp1_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00001u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod trn1_z_zz_q {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct trn1_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl trn1_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00011u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod zip2_z_zz_q {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zip2_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl zip2_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00000u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod uzp2_z_zz_q {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uzp2_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uzp2_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00001u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod trn2_z_zz_q {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct trn2_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl trn2_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00011u32 << 11u32
                    | self.H.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
