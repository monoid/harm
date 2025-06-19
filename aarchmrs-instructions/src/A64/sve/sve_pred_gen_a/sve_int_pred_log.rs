/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod and_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct and_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl and_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod bic_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bic_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl bic_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod orr_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct orr_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl orr_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod orn_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct orn_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl orn_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod eor_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct eor_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl eor_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b1u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod sel_p_p_pp_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sel_p_p_pp_ {
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl sel_p_p_pp_ {
        #[inline]
        pub const fn new(
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010000u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b1u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod nor_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct nor_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl nor_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b1u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod nand_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct nand_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl nand_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b1u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod ands_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ands_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl ands_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod bics_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bics_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl bics_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod orrs_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct orrs_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl orrs_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod orns_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct orns_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl orns_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod eors_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct eors_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl eors_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b1u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod nors_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct nors_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl nors_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b1u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
pub mod nands_p_p_pp_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct nands_p_p_pp_z {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl nands_p_p_pp_z {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { S, Pm, Pg, Pn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b1u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
            )
        }
    }
}
