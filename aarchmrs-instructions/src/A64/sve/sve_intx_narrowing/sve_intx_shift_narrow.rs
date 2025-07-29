/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqshrunb_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqshrunb_z_zi_";
    #[inline]
    pub const fn sqshrunb_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b00000u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshrunb_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrunb_z_zi_";
    #[inline]
    pub const fn sqrshrunb_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b00001u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod shrnb_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "shrnb_z_zi_";
    #[inline]
    pub const fn shrnb_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b00010u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod rshrnb_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000001100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "rshrnb_z_zi_";
    #[inline]
    pub const fn rshrnb_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b00011u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqshrnb_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqshrnb_z_zi_";
    #[inline]
    pub const fn sqshrnb_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b001u32 << 13u32
                | U.into_inner() << 12u32
                | 0b0u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshrnb_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrnb_z_zi_";
    #[inline]
    pub const fn sqrshrnb_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b001u32 << 13u32
                | U.into_inner() << 12u32
                | 0b1u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqshrunt_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqshrunt_z_zi_";
    #[inline]
    pub const fn sqshrunt_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b00000u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshrunt_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrunt_z_zi_";
    #[inline]
    pub const fn sqrshrunt_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b00001u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod shrnt_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "shrnt_z_zi_";
    #[inline]
    pub const fn shrnt_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b00010u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod rshrnt_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000001100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "rshrnt_z_zi_";
    #[inline]
    pub const fn rshrnt_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b00011u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqshrnt_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqshrnt_z_zi_";
    #[inline]
    pub const fn sqshrnt_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b001u32 << 13u32
                | U.into_inner() << 12u32
                | 0b0u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshrnt_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrnt_z_zi_";
    #[inline]
    pub const fn sqrshrnt_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b001u32 << 13u32
                | U.into_inner() << 12u32
                | 0b1u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqshrnb_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqshrnb_z_zi_";
    #[inline]
    pub const fn uqshrnb_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b001u32 << 13u32
                | U.into_inner() << 12u32
                | 0b0u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqrshrnb_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqrshrnb_z_zi_";
    #[inline]
    pub const fn uqrshrnb_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b001u32 << 13u32
                | U.into_inner() << 12u32
                | 0b1u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqshrnt_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqshrnt_z_zi_";
    #[inline]
    pub const fn uqshrnt_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b001u32 << 13u32
                | U.into_inner() << 12u32
                | 0b0u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqrshrnt_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqrshrnt_z_zi_";
    #[inline]
    pub const fn uqrshrnt_z_zi_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b001u32 << 13u32
                | U.into_inner() << 12u32
                | 0b1u32 << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
