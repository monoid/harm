/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ftmopa_za32_zzzi_h2x1 {
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000001100u32;
    pub const OPCODE: u32 = 0b10000001011000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ftmopa_za32_zzzi_h2x1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ftmopa_za32_zzzi_h2x1 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub K: ::aarchmrs_types::BitValue<1>,
        pub Zk: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl ftmopa_za32_zzzi_h2x1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            K: ::aarchmrs_types::BitValue<1>,
            Zk: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            i2: ::aarchmrs_types::BitValue<2>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                K,
                Zk,
                Zn,
                i2,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b000u32 << 13u32
                    | self.K.into_inner() << 12u32
                    | self.Zk.into_inner() << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.i2.into_inner() << 4u32
                    | 0b00u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
