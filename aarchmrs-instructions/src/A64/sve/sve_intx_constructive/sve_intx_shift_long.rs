/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sshllb_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sshllb_z_zi_";
    #[inline]
    pub const fn sshllb_z_zi_(
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
                | 0b0u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sshllt_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sshllt_z_zi_";
    #[inline]
    pub const fn sshllt_z_zi_(
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
                | 0b0u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod ushllb_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ushllb_z_zi_";
    #[inline]
    pub const fn ushllb_z_zi_(
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
                | 0b0u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod ushllt_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ushllt_z_zi_";
    #[inline]
    pub const fn ushllt_z_zi_(
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
                | 0b0u32 << 21u32
                | tszl.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
