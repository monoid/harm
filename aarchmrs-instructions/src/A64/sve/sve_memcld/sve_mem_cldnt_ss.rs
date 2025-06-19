/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldnt1b_z_p_br_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1b_z_p_br_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1b_z_p_br_contiguous {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                Rm,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | 0b00u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldnt1h_z_p_br_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1h_z_p_br_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1h_z_p_br_contiguous {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                Rm,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | 0b00u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldnt1w_z_p_br_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1w_z_p_br_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1w_z_p_br_contiguous {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                Rm,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | 0b00u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldnt1d_z_p_br_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1d_z_p_br_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1d_z_p_br_contiguous {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                Rm,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | 0b00u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
