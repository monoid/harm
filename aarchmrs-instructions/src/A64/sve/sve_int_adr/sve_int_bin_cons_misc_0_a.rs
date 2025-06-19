/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod adr_z_az_d_s32_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct adr_z_az_d_s32_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl adr_z_az_d_s32_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, msz, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100001u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1010u32 << 12u32
                    | self.msz.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod adr_z_az_d_u32_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct adr_z_az_d_u32_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl adr_z_az_d_u32_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, msz, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1010u32 << 12u32
                    | self.msz.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod adr_z_az_sd_same_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct adr_z_az_sd_same_scaled {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl adr_z_az_sd_same_scaled {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                sz,
                Zm,
                msz,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000001001u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1010u32 << 12u32
                    | self.msz.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
