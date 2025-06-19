/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mad_z_p_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mad_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl mad_z_p_zzz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Za: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                Zm,
                Pg,
                Za,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Za.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod msb_z_p_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct msb_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl msb_z_p_zzz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Za: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                Zm,
                Pg,
                Za,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b111u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Za.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
