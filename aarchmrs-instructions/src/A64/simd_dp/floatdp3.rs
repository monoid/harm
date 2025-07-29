/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FMADD_S_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMADD_S_floatdp3";
    #[inline]
    pub const fn FMADD_S_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMSUB_S_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMSUB_S_floatdp3";
    #[inline]
    pub const fn FMSUB_S_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FNMADD_S_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FNMADD_S_floatdp3";
    #[inline]
    pub const fn FNMADD_S_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FNMSUB_S_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FNMSUB_S_floatdp3";
    #[inline]
    pub const fn FNMSUB_S_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMADD_D_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMADD_D_floatdp3";
    #[inline]
    pub const fn FMADD_D_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMSUB_D_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMSUB_D_floatdp3";
    #[inline]
    pub const fn FMSUB_D_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FNMADD_D_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FNMADD_D_floatdp3";
    #[inline]
    pub const fn FNMADD_D_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FNMSUB_D_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111011000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FNMSUB_D_floatdp3";
    #[inline]
    pub const fn FNMSUB_D_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMADD_H_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMADD_H_floatdp3";
    #[inline]
    pub const fn FMADD_H_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMSUB_H_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111110000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMSUB_H_floatdp3";
    #[inline]
    pub const fn FMSUB_H_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FNMADD_H_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FNMADD_H_floatdp3";
    #[inline]
    pub const fn FNMADD_H_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111111u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FNMSUB_H_floatdp3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FNMSUB_H_floatdp3";
    #[inline]
    pub const fn FNMSUB_H_floatdp3(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011111111u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
