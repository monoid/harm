/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod DUP_asimdins_DV_v {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DUP_asimdins_DV_v";
    #[inline]
    pub const fn DUP_asimdins_DV_v(
        Q: ::aarchmrs_types::BitValue<1>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110000u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b000001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod DUP_asimdins_DR_r {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DUP_asimdins_DR_r";
    #[inline]
    pub const fn DUP_asimdins_DR_r(
        Q: ::aarchmrs_types::BitValue<1>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110000u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b000011u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SMOV_asimdins_W_w {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000010110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMOV_asimdins_W_w";
    #[inline]
    pub const fn SMOV_asimdins_W_w(
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001110000u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b001011u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UMOV_asimdins_W_w {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000011110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMOV_asimdins_W_w";
    #[inline]
    pub const fn UMOV_asimdins_W_w(
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001110000u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b001111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod INS_asimdins_IR_r {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001110000000000001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "INS_asimdins_IR_r";
    #[inline]
    pub const fn INS_asimdins_IR_r(
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001110000u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b000111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SMOV_asimdins_X_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001110000000000010110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMOV_asimdins_X_x";
    #[inline]
    pub const fn SMOV_asimdins_X_x(
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001110000u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b001011u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UMOV_asimdins_X_x {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111011111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001110000010000011110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMOV_asimdins_X_x";
    #[inline]
    pub const fn UMOV_asimdins_X_x(
        imm5: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001110000u32 << 21u32
                | imm5.into_inner() << 20u32
                | 0b1000001111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod INS_asimdins_IV_v {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101110000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "INS_asimdins_IV_v";
    #[inline]
    pub const fn INS_asimdins_IV_v(
        imm5: ::aarchmrs_types::BitValue<5>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01101110000u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm4.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
