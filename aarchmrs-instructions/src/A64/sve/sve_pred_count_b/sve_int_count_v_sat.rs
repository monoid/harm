/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincp_z_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincp_z_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincp_z_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, U, Pm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10100u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b1000000u32 << 9u32
                    | self.Pm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqdecp_z_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecp_z_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecp_z_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, U, Pm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10101u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b1000000u32 << 9u32
                    | self.Pm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod uqincp_z_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincp_z_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincp_z_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, U, Pm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10100u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b1000000u32 << 9u32
                    | self.Pm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod uqdecp_z_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecp_z_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecp_z_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, U, Pm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10101u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b1000000u32 << 9u32
                    | self.Pm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
