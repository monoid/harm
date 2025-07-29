/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CSEL_32_condsel {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CSEL_32_condsel";
    #[inline]
    pub const fn CSEL_32_condsel(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010100u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b0u32 << 11u32
                | o2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CSINC_32_condsel {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CSINC_32_condsel";
    #[inline]
    pub const fn CSINC_32_condsel(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010100u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b0u32 << 11u32
                | o2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CSINV_32_condsel {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CSINV_32_condsel";
    #[inline]
    pub const fn CSINV_32_condsel(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011010100u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b0u32 << 11u32
                | o2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CSNEG_32_condsel {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CSNEG_32_condsel";
    #[inline]
    pub const fn CSNEG_32_condsel(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011010100u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b0u32 << 11u32
                | o2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CSEL_64_condsel {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CSEL_64_condsel";
    #[inline]
    pub const fn CSEL_64_condsel(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010100u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b0u32 << 11u32
                | o2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CSINC_64_condsel {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CSINC_64_condsel";
    #[inline]
    pub const fn CSINC_64_condsel(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010100u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b0u32 << 11u32
                | o2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CSINV_64_condsel {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CSINV_64_condsel";
    #[inline]
    pub const fn CSINV_64_condsel(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010100u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b0u32 << 11u32
                | o2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CSNEG_64_condsel {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CSNEG_64_condsel";
    #[inline]
    pub const fn CSNEG_64_condsel(
        Rm: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        o2: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010100u32 << 21u32
                | Rm.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b0u32 << 11u32
                | o2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
