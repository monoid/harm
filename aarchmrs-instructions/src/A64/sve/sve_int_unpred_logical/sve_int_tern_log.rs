/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod eor3_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct eor3_z_zzz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zk: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl eor3_z_zzz_ {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Zk: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, Zk, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100001u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b001110u32 << 10u32
                    | self.Zk.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod bcax_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bcax_z_zzz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zk: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl bcax_z_zzz_ {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Zk: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, Zk, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b001110u32 << 10u32
                    | self.Zk.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod bsl_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bsl_z_zzz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zk: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl bsl_z_zzz_ {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Zk: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, Zk, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100001u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b001111u32 << 10u32
                    | self.Zk.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod bsl1n_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bsl1n_z_zzz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zk: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl bsl1n_z_zzz_ {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Zk: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, Zk, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b001111u32 << 10u32
                    | self.Zk.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod bsl2n_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bsl2n_z_zzz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zk: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl bsl2n_z_zzz_ {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Zk: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, Zk, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b001111u32 << 10u32
                    | self.Zk.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod nbsl_z_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct nbsl_z_zzz_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zk: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl nbsl_z_zzz_ {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Zk: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, Zk, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b001111u32 << 10u32
                    | self.Zk.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
