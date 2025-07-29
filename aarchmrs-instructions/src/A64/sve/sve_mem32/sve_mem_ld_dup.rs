/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1rb_z_p_bi_u8 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rb_z_p_bi_u8";
    #[inline]
    pub const fn ld1rb_z_p_bi_u8(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rb_z_p_bi_u16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rb_z_p_bi_u16";
    #[inline]
    pub const fn ld1rb_z_p_bi_u16(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rb_z_p_bi_u32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rb_z_p_bi_u32";
    #[inline]
    pub const fn ld1rb_z_p_bi_u32(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rb_z_p_bi_u64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rb_z_p_bi_u64";
    #[inline]
    pub const fn ld1rb_z_p_bi_u64(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rsw_z_p_bi_s64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rsw_z_p_bi_s64";
    #[inline]
    pub const fn ld1rsw_z_p_bi_s64(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rh_z_p_bi_u16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rh_z_p_bi_u16";
    #[inline]
    pub const fn ld1rh_z_p_bi_u16(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rh_z_p_bi_u32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rh_z_p_bi_u32";
    #[inline]
    pub const fn ld1rh_z_p_bi_u32(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rh_z_p_bi_u64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rh_z_p_bi_u64";
    #[inline]
    pub const fn ld1rh_z_p_bi_u64(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rsh_z_p_bi_s64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rsh_z_p_bi_s64";
    #[inline]
    pub const fn ld1rsh_z_p_bi_s64(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rsh_z_p_bi_s32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rsh_z_p_bi_s32";
    #[inline]
    pub const fn ld1rsh_z_p_bi_s32(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rw_z_p_bi_u32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rw_z_p_bi_u32";
    #[inline]
    pub const fn ld1rw_z_p_bi_u32(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rw_z_p_bi_u64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rw_z_p_bi_u64";
    #[inline]
    pub const fn ld1rw_z_p_bi_u64(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rsb_z_p_bi_s64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rsb_z_p_bi_s64";
    #[inline]
    pub const fn ld1rsb_z_p_bi_s64(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rsb_z_p_bi_s32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rsb_z_p_bi_s32";
    #[inline]
    pub const fn ld1rsb_z_p_bi_s32(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rsb_z_p_bi_s16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rsb_z_p_bi_s16";
    #[inline]
    pub const fn ld1rsb_z_p_bi_s16(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1rd_z_p_bi_u64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1rd_z_p_bi_u64";
    #[inline]
    pub const fn ld1rd_z_p_bi_u64(
        dtypeh: ::aarchmrs_types::BitValue<2>,
        imm6: ::aarchmrs_types::BitValue<6>,
        dtypel: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | dtypeh.into_inner() << 23u32
                | 0b1u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b1u32 << 15u32
                | dtypel.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
