/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod and_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "and_p_p_pp_z";
    #[inline]
    pub const fn and_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod bic_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000100000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bic_p_p_pp_z";
    #[inline]
    pub const fn bic_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod orr_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101100000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "orr_p_p_pp_z";
    #[inline]
    pub const fn orr_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod orn_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101100000000100000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "orn_p_p_pp_z";
    #[inline]
    pub const fn orn_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod eor_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000100001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "eor_p_p_pp_z";
    #[inline]
    pub const fn eor_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b1u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod sel_p_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000100001000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sel_p_p_pp_";
    #[inline]
    pub const fn sel_p_p_pp_(
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010000u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b1u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod nor_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101100000000100001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "nor_p_p_pp_z";
    #[inline]
    pub const fn nor_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b1u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod nand_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101100000000100001000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "nand_p_p_pp_z";
    #[inline]
    pub const fn nand_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b1u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod ands_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ands_p_p_pp_z";
    #[inline]
    pub const fn ands_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod bics_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000100000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bics_p_p_pp_z";
    #[inline]
    pub const fn bics_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod orrs_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101100000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "orrs_p_p_pp_z";
    #[inline]
    pub const fn orrs_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod orns_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101100000000100000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "orns_p_p_pp_z";
    #[inline]
    pub const fn orns_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod eors_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000000000100001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "eors_p_p_pp_z";
    #[inline]
    pub const fn eors_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b1u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod nors_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101100000000100001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "nors_p_p_pp_z";
    #[inline]
    pub const fn nors_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b1u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod nands_p_p_pp_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101100000000100001000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "nands_p_p_pp_z";
    #[inline]
    pub const fn nands_p_p_pp_z(
        S: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | S.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b1u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
