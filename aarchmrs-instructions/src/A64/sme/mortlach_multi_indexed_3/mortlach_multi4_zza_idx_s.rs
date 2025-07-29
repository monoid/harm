/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmla_za_zzi_s4xi";
    #[inline]
    pub const fn fmla_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0000u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fdot_za32_z8z8i_4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fdot_za32_z8z8i_4xi";
    #[inline]
    pub const fn fdot_za32_z8z8i_4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0001u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod svdot_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001000000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "svdot_za_zzi_s4xi";
    #[inline]
    pub const fn svdot_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0100u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod usvdot_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001000000000101000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usvdot_za_zzi_s4xi";
    #[inline]
    pub const fn usvdot_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0101u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod sdot_za32_zzi_4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sdot_za32_zzi_4xi";
    #[inline]
    pub const fn sdot_za32_zzi_4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0000u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fdot_za_zzi_4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001001000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fdot_za_zzi_4xi";
    #[inline]
    pub const fn fdot_za_zzi_4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0001u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod bfdot_za_zzi_4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001001000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfdot_za_zzi_4xi";
    #[inline]
    pub const fn bfdot_za_zzi_4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0011u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod sdot_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001001000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sdot_za_zzi_s4xi";
    #[inline]
    pub const fn sdot_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0100u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod usdot_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001001000000101000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usdot_za_zzi_s4xi";
    #[inline]
    pub const fn usdot_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0101u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fmls_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmls_za_zzi_s4xi";
    #[inline]
    pub const fn fmls_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0010u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod uvdot_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uvdot_za_zzi_s4xi";
    #[inline]
    pub const fn uvdot_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0110u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod suvdot_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001000000000111000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "suvdot_za_zzi_s4xi";
    #[inline]
    pub const fn suvdot_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0111u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod udot_za32_zzi_4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001001000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "udot_za32_zzi_4xi";
    #[inline]
    pub const fn udot_za32_zzi_4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0010u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod udot_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001001000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "udot_za_zzi_s4xi";
    #[inline]
    pub const fn udot_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0110u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod sudot_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001010100001001000000111000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sudot_za_zzi_s4xi";
    #[inline]
    pub const fn sudot_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i2: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i2.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0111u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
