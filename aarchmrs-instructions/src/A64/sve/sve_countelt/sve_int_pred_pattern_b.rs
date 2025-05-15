/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincb_r_rs_sx {
    #[inline]
    pub fn sqincb_r_rs_sx(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqincb_r_rs_uw {
    #[inline]
    pub fn uqincb_r_rs_uw(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqdecb_r_rs_sx {
    #[inline]
    pub fn sqdecb_r_rs_sx(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111110u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqdecb_r_rs_uw {
    #[inline]
    pub fn uqdecb_r_rs_uw(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqincb_r_rs_x {
    #[inline]
    pub fn sqincb_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11110u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqdecb_r_rs_x {
    #[inline]
    pub fn sqdecb_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11111u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqinch_r_rs_sx {
    #[inline]
    pub fn sqinch_r_rs_sx(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqinch_r_rs_uw {
    #[inline]
    pub fn uqinch_r_rs_uw(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqdech_r_rs_sx {
    #[inline]
    pub fn sqdech_r_rs_sx(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111110u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqdech_r_rs_uw {
    #[inline]
    pub fn uqdech_r_rs_uw(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqinch_r_rs_x {
    #[inline]
    pub fn sqinch_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11110u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqdech_r_rs_x {
    #[inline]
    pub fn sqdech_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11111u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqincw_r_rs_sx {
    #[inline]
    pub fn sqincw_r_rs_sx(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqincw_r_rs_uw {
    #[inline]
    pub fn uqincw_r_rs_uw(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqdecw_r_rs_sx {
    #[inline]
    pub fn sqdecw_r_rs_sx(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111110u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqdecw_r_rs_uw {
    #[inline]
    pub fn uqdecw_r_rs_uw(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqincw_r_rs_x {
    #[inline]
    pub fn sqincw_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11110u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqdecw_r_rs_x {
    #[inline]
    pub fn sqdecw_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11111u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqincd_r_rs_sx {
    #[inline]
    pub fn sqincd_r_rs_sx(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111100u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqincd_r_rs_uw {
    #[inline]
    pub fn uqincd_r_rs_uw(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqdecd_r_rs_sx {
    #[inline]
    pub fn sqdecd_r_rs_sx(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111110u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqdecd_r_rs_uw {
    #[inline]
    pub fn uqdecd_r_rs_uw(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqincd_r_rs_x {
    #[inline]
    pub fn sqincd_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11110u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod sqdecd_r_rs_x {
    #[inline]
    pub fn sqdecd_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11111u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqincb_r_rs_x {
    #[inline]
    pub fn uqincb_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11110u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqdecb_r_rs_x {
    #[inline]
    pub fn uqdecb_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11111u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqinch_r_rs_x {
    #[inline]
    pub fn uqinch_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11110u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqdech_r_rs_x {
    #[inline]
    pub fn uqdech_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11111u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqincw_r_rs_x {
    #[inline]
    pub fn uqincw_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11110u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqdecw_r_rs_x {
    #[inline]
    pub fn uqdecw_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11111u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqincd_r_rs_x {
    #[inline]
    pub fn uqincd_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11110u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod uqdecd_r_rs_x {
    #[inline]
    pub fn uqdecd_r_rs_x(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11111u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
