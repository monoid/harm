/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RBIT_32_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011010110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RBIT_32_dp_1src";
    #[inline]
    pub const fn RBIT_32_dp_1src(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101101011000000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod REV16_32_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011010110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV16_32_dp_1src";
    #[inline]
    pub const fn REV16_32_dp_1src(
        opc: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011010110000000000u32 << 12u32
                | opc.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod REV_32_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011010110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV_32_dp_1src";
    #[inline]
    pub const fn REV_32_dp_1src(
        opc: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011010110000000000u32 << 12u32
                | opc.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CLZ_32_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011010110000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLZ_32_dp_1src";
    #[inline]
    pub const fn CLZ_32_dp_1src(
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010110101100000000010u32 << 11u32
                | op.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CLS_32_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011010110000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLS_32_dp_1src";
    #[inline]
    pub const fn CLS_32_dp_1src(
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010110101100000000010u32 << 11u32
                | op.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CTZ_32_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011010110000000001100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CTZ_32_dp_1src";
    #[inline]
    pub const fn CTZ_32_dp_1src(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101101011000000000110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CNT_32_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011010110000000001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CNT_32_dp_1src";
    #[inline]
    pub const fn CNT_32_dp_1src(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101101011000000000111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ABS_32_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011010110000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ABS_32_dp_1src";
    #[inline]
    pub const fn ABS_32_dp_1src(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101101011000000001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod RBIT_64_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RBIT_64_dp_1src";
    #[inline]
    pub const fn RBIT_64_dp_1src(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod REV16_64_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV16_64_dp_1src";
    #[inline]
    pub const fn REV16_64_dp_1src(
        opc: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010110000000000u32 << 12u32
                | opc.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod REV32_64_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV32_64_dp_1src";
    #[inline]
    pub const fn REV32_64_dp_1src(
        opc: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010110000000000u32 << 12u32
                | opc.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod REV_64_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV_64_dp_1src";
    #[inline]
    pub const fn REV_64_dp_1src(
        opc: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010110000000000u32 << 12u32
                | opc.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CLZ_64_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLZ_64_dp_1src";
    #[inline]
    pub const fn CLZ_64_dp_1src(
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000000010u32 << 11u32
                | op.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CLS_64_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLS_64_dp_1src";
    #[inline]
    pub const fn CLS_64_dp_1src(
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000000010u32 << 11u32
                | op.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CTZ_64_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000000001100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CTZ_64_dp_1src";
    #[inline]
    pub const fn CTZ_64_dp_1src(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000000000110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CNT_64_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000000001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CNT_64_dp_1src";
    #[inline]
    pub const fn CNT_64_dp_1src(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000000000111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ABS_64_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ABS_64_dp_1src";
    #[inline]
    pub const fn ABS_64_dp_1src(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000000001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod PACIA_64P_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACIA_64P_dp_1src";
    #[inline]
    pub const fn PACIA_64P_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod PACIB_64P_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACIB_64P_dp_1src";
    #[inline]
    pub const fn PACIB_64P_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod PACDA_64P_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACDA_64P_dp_1src";
    #[inline]
    pub const fn PACDA_64P_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod PACDB_64P_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACDB_64P_dp_1src";
    #[inline]
    pub const fn PACDB_64P_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b011u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod AUTIA_64P_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTIA_64P_dp_1src";
    #[inline]
    pub const fn AUTIA_64P_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b100u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod AUTIB_64P_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTIB_64P_dp_1src";
    #[inline]
    pub const fn AUTIB_64P_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod AUTDA_64P_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010001100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTDA_64P_dp_1src";
    #[inline]
    pub const fn AUTDA_64P_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod AUTDB_64P_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTDB_64P_dp_1src";
    #[inline]
    pub const fn AUTDB_64P_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod PACIZA_64Z_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010000001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACIZA_64Z_dp_1src";
    #[inline]
    pub const fn PACIZA_64Z_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b00011111u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod PACIZB_64Z_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010000011111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACIZB_64Z_dp_1src";
    #[inline]
    pub const fn PACIZB_64Z_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b00111111u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod PACDZA_64Z_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010000101111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACDZA_64Z_dp_1src";
    #[inline]
    pub const fn PACDZA_64Z_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b01011111u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod PACDZB_64Z_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010000111111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACDZB_64Z_dp_1src";
    #[inline]
    pub const fn PACDZB_64Z_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b01111111u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod AUTIZA_64Z_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010001001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTIZA_64Z_dp_1src";
    #[inline]
    pub const fn AUTIZA_64Z_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b10011111u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod AUTIZB_64Z_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010001011111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTIZB_64Z_dp_1src";
    #[inline]
    pub const fn AUTIZB_64Z_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b10111111u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod AUTDZA_64Z_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010001101111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTDZA_64Z_dp_1src";
    #[inline]
    pub const fn AUTDZA_64Z_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b11011111u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod AUTDZB_64Z_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010001111111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTDZB_64Z_dp_1src";
    #[inline]
    pub const fn AUTDZB_64Z_dp_1src(
        Z: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | Z.into_inner() << 13u32
                | 0b11111111u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod XPACI_64Z_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111101111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010100001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "XPACI_64Z_dp_1src";
    #[inline]
    pub const fn XPACI_64Z_dp_1src(
        D: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000101000u32 << 11u32
                | D.into_inner() << 10u32
                | 0b11111u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod XPACD_64Z_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111101111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000010100001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "XPACD_64Z_dp_1src";
    #[inline]
    pub const fn XPACD_64Z_dp_1src(
        D: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000101000u32 << 11u32
                | D.into_inner() << 10u32
                | 0b11111u32 << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod PACNBIASPPC_64LR_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000011000001111111110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACNBIASPPC_64LR_dp_1src";
    #[inline]
    pub const fn PACNBIASPPC_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011000001111111110u32 << 0u32)
    }
}
pub mod PACNBIBSPPC_64LR_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000011000011111111110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACNBIBSPPC_64LR_dp_1src";
    #[inline]
    pub const fn PACNBIBSPPC_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011000011111111110u32 << 0u32)
    }
}
pub mod PACIA171615_64LR_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000011000101111111110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACIA171615_64LR_dp_1src";
    #[inline]
    pub const fn PACIA171615_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011000101111111110u32 << 0u32)
    }
}
pub mod PACIB171615_64LR_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000011000111111111110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACIB171615_64LR_dp_1src";
    #[inline]
    pub const fn PACIB171615_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011000111111111110u32 << 0u32)
    }
}
pub mod AUTIASPPCR_64LRR_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000011001000000011110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTIASPPCR_64LRR_dp_1src";
    #[inline]
    pub const fn AUTIASPPCR_64LRR_dp_1src(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000001100100u32 << 10u32 | Rn.into_inner() << 5u32 | 0b11110u32 << 0u32,
        )
    }
}
pub mod AUTIBSPPCR_64LRR_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000011001010000011110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTIBSPPCR_64LRR_dp_1src";
    #[inline]
    pub const fn AUTIBSPPCR_64LRR_dp_1src(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000001100101u32 << 10u32 | Rn.into_inner() << 5u32 | 0b11110u32 << 0u32,
        )
    }
}
pub mod PACIASPPC_64LR_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000011010001111111110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACIASPPC_64LR_dp_1src";
    #[inline]
    pub const fn PACIASPPC_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011010001111111110u32 << 0u32)
    }
}
pub mod PACIBSPPC_64LR_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000011010011111111110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PACIBSPPC_64LR_dp_1src";
    #[inline]
    pub const fn PACIBSPPC_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011010011111111110u32 << 0u32)
    }
}
pub mod AUTIA171615_64LR_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000011011101111111110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTIA171615_64LR_dp_1src";
    #[inline]
    pub const fn AUTIA171615_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011011101111111110u32 << 0u32)
    }
}
pub mod AUTIB171615_64LR_dp_1src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011010110000011011111111111110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AUTIB171615_64LR_dp_1src";
    #[inline]
    pub const fn AUTIB171615_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011011111111111110u32 << 0u32)
    }
}
