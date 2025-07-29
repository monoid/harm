/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STLLRB_SL32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000100111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLLRB_SL32_ldstord";
    #[inline]
    pub const fn STLLRB_SL32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0000100010011111011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLRB_SL32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000100111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLRB_SL32_ldstord";
    #[inline]
    pub const fn STLRB_SL32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0000100010011111111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDLARB_LR32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000110111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDLARB_LR32_ldstord";
    #[inline]
    pub const fn LDLARB_LR32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0000100011011111011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDARB_LR32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001000110111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDARB_LR32_ldstord";
    #[inline]
    pub const fn LDARB_LR32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0000100011011111111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLLRH_SL32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001000100111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLLRH_SL32_ldstord";
    #[inline]
    pub const fn STLLRH_SL32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100100010011111011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLRH_SL32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001000100111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLRH_SL32_ldstord";
    #[inline]
    pub const fn STLRH_SL32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100100010011111111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDLARH_LR32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001000110111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDLARH_LR32_ldstord";
    #[inline]
    pub const fn LDLARH_LR32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100100011011111011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDARH_LR32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001000110111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDARH_LR32_ldstord";
    #[inline]
    pub const fn LDARH_LR32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100100011011111111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLLR_SL32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001000100111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLLR_SL32_ldstord";
    #[inline]
    pub const fn STLLR_SL32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000100010011111011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLR_SL32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001000100111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLR_SL32_ldstord";
    #[inline]
    pub const fn STLR_SL32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000100010011111111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDLAR_LR32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001000110111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDLAR_LR32_ldstord";
    #[inline]
    pub const fn LDLAR_LR32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000100011011111011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDAR_LR32_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001000110111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAR_LR32_ldstord";
    #[inline]
    pub const fn LDAR_LR32_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000100011011111111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLLR_SL64_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001000100111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLLR_SL64_ldstord";
    #[inline]
    pub const fn STLLR_SL64_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100100010011111011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLR_SL64_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001000100111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLR_SL64_ldstord";
    #[inline]
    pub const fn STLR_SL64_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100100010011111111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDLAR_LR64_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001000110111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDLAR_LR64_ldstord";
    #[inline]
    pub const fn LDLAR_LR64_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100100011011111011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDAR_LR64_ldstord {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001000110111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAR_LR64_ldstord";
    #[inline]
    pub const fn LDAR_LR64_ldstord(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100100011011111111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
