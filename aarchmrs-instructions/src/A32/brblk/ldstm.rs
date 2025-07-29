/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STMDA_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STMDA_A1";
    #[inline]
    pub const fn STMDA_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100000u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod LDMDA_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDMDA_A1";
    #[inline]
    pub const fn LDMDA_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100000u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod STM_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STM_A1";
    #[inline]
    pub const fn STM_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100010u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod LDM_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000100100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDM_A1";
    #[inline]
    pub const fn LDM_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100010u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod STM_u_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001110011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STM_u_A1_AS";
    #[inline]
    pub const fn STM_u_A1_AS(
        cond: ::aarchmrs_types::BitValue<4>,
        P: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100u32 << 25u32
                | P.into_inner() << 24u32
                | U.into_inner() << 23u32
                | 0b100u32 << 20u32
                | Rn.into_inner() << 16u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod STMDB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001001000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STMDB_A1";
    #[inline]
    pub const fn STMDB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100100u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod LDMDB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001001000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDMDB_A1";
    #[inline]
    pub const fn LDMDB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100100u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod LDM_u_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001110011100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDM_u_A1_AS";
    #[inline]
    pub const fn LDM_u_A1_AS(
        cond: ::aarchmrs_types::BitValue<4>,
        P: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<15>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100u32 << 25u32
                | P.into_inner() << 24u32
                | U.into_inner() << 23u32
                | 0b101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod STMIB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001001100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STMIB_A1";
    #[inline]
    pub const fn STMIB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100110u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod LDMIB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001001100100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDMIB_A1";
    #[inline]
    pub const fn LDMIB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100110u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod LDM_e_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001110010100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000010100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDM_e_A1_AS";
    #[inline]
    pub const fn LDM_e_A1_AS(
        cond: ::aarchmrs_types::BitValue<4>,
        P: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        register_list: ::aarchmrs_types::BitValue<15>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b100u32 << 25u32
                | P.into_inner() << 24u32
                | U.into_inner() << 23u32
                | 0b1u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1u32 << 15u32
                | register_list.into_inner() << 0u32,
        )
    }
}
