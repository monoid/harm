/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod saddlb_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "saddlb_z_zz_";
    #[inline]
    pub const fn saddlb_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod ssublb_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ssublb_z_zz_";
    #[inline]
    pub const fn ssublb_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0001u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sabdlb_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sabdlb_z_zz_";
    #[inline]
    pub const fn sabdlb_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0011u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod saddlt_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "saddlt_z_zz_";
    #[inline]
    pub const fn saddlt_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod ssublt_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ssublt_z_zz_";
    #[inline]
    pub const fn ssublt_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0001u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sabdlt_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sabdlt_z_zz_";
    #[inline]
    pub const fn sabdlt_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0011u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uaddlb_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uaddlb_z_zz_";
    #[inline]
    pub const fn uaddlb_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod usublb_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usublb_z_zz_";
    #[inline]
    pub const fn usublb_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0001u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uabdlb_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uabdlb_z_zz_";
    #[inline]
    pub const fn uabdlb_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0011u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uaddlt_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uaddlt_z_zz_";
    #[inline]
    pub const fn uaddlt_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod usublt_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usublt_z_zz_";
    #[inline]
    pub const fn usublt_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0001u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uabdlt_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000000011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uabdlt_z_zz_";
    #[inline]
    pub const fn uabdlt_z_zz_(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b0011u32 << 12u32
                | U.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
