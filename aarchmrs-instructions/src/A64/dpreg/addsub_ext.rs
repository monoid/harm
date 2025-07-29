/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_32_addsub_ext {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_32_addsub_ext";
    #[inline]
    pub const fn ADD_32_addsub_ext(
        Rm: ::aarchmrs_types::BitValue<5>,
        option: ::aarchmrs_types::BitValue<3>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001011001u32 << 21u32
                | Rm.into_inner() << 16u32
                | option.into_inner() << 13u32
                | imm3.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ADDS_32S_addsub_ext {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_32S_addsub_ext";
    #[inline]
    pub const fn ADDS_32S_addsub_ext(
        Rm: ::aarchmrs_types::BitValue<5>,
        option: ::aarchmrs_types::BitValue<3>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00101011001u32 << 21u32
                | Rm.into_inner() << 16u32
                | option.into_inner() << 13u32
                | imm3.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SUB_32_addsub_ext {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_32_addsub_ext";
    #[inline]
    pub const fn SUB_32_addsub_ext(
        Rm: ::aarchmrs_types::BitValue<5>,
        option: ::aarchmrs_types::BitValue<3>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001011001u32 << 21u32
                | Rm.into_inner() << 16u32
                | option.into_inner() << 13u32
                | imm3.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SUBS_32S_addsub_ext {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_32S_addsub_ext";
    #[inline]
    pub const fn SUBS_32S_addsub_ext(
        Rm: ::aarchmrs_types::BitValue<5>,
        option: ::aarchmrs_types::BitValue<3>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01101011001u32 << 21u32
                | Rm.into_inner() << 16u32
                | option.into_inner() << 13u32
                | imm3.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ADD_64_addsub_ext {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_64_addsub_ext";
    #[inline]
    pub const fn ADD_64_addsub_ext(
        Rm: ::aarchmrs_types::BitValue<5>,
        option: ::aarchmrs_types::BitValue<3>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001011001u32 << 21u32
                | Rm.into_inner() << 16u32
                | option.into_inner() << 13u32
                | imm3.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ADDS_64S_addsub_ext {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_64S_addsub_ext";
    #[inline]
    pub const fn ADDS_64S_addsub_ext(
        Rm: ::aarchmrs_types::BitValue<5>,
        option: ::aarchmrs_types::BitValue<3>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10101011001u32 << 21u32
                | Rm.into_inner() << 16u32
                | option.into_inner() << 13u32
                | imm3.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SUB_64_addsub_ext {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_64_addsub_ext";
    #[inline]
    pub const fn SUB_64_addsub_ext(
        Rm: ::aarchmrs_types::BitValue<5>,
        option: ::aarchmrs_types::BitValue<3>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001011001u32 << 21u32
                | Rm.into_inner() << 16u32
                | option.into_inner() << 13u32
                | imm3.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SUBS_64S_addsub_ext {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_64S_addsub_ext";
    #[inline]
    pub const fn SUBS_64S_addsub_ext(
        Rm: ::aarchmrs_types::BitValue<5>,
        option: ::aarchmrs_types::BitValue<3>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011001u32 << 21u32
                | Rm.into_inner() << 16u32
                | option.into_inner() << 13u32
                | imm3.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
