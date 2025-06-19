/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod aese_mz_zzi_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aese_mz_zzi_2x1 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl aese_mz_zzi_2x1 {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { i2, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001u32 << 21u32
                    | self.i2.into_inner() << 19u32
                    | 0b010111010u32 << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod aesd_mz_zzi_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aesd_mz_zzi_2x1 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl aesd_mz_zzi_2x1 {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { i2, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001u32 << 21u32
                    | self.i2.into_inner() << 19u32
                    | 0b010111011u32 << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod aesemc_mz_zzi_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aesemc_mz_zzi_2x1 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl aesemc_mz_zzi_2x1 {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { i2, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001u32 << 21u32
                    | self.i2.into_inner() << 19u32
                    | 0b011111010u32 << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod aesdimc_mz_zzi_2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aesdimc_mz_zzi_2x1 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl aesdimc_mz_zzi_2x1 {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { i2, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001u32 << 21u32
                    | self.i2.into_inner() << 19u32
                    | 0b011111011u32 << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
