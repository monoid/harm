/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smop4a_za_zz_h1x1 {
    #[inline]
    pub fn smop4a_za_zz_h1x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_h1x1 {
    #[inline]
    pub fn sumop4a_za_zz_h1x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_h1x1 {
    #[inline]
    pub fn usmop4a_za_zz_h1x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod umop4a_za_zz_h1x1 {
    #[inline]
    pub fn umop4a_za_zz_h1x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod smop4s_za_zz_h1x1 {
    #[inline]
    pub fn smop4s_za_zz_h1x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_h1x1 {
    #[inline]
    pub fn sumop4s_za_zz_h1x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_h1x1 {
    #[inline]
    pub fn usmop4s_za_zz_h1x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod umop4s_za_zz_h1x1 {
    #[inline]
    pub fn umop4s_za_zz_h1x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod smop4a_za_zz_h1x2 {
    #[inline]
    pub fn smop4a_za_zz_h1x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_h1x2 {
    #[inline]
    pub fn sumop4a_za_zz_h1x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_h1x2 {
    #[inline]
    pub fn usmop4a_za_zz_h1x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod umop4a_za_zz_h1x2 {
    #[inline]
    pub fn umop4a_za_zz_h1x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod smop4s_za_zz_h1x2 {
    #[inline]
    pub fn smop4s_za_zz_h1x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_h1x2 {
    #[inline]
    pub fn sumop4s_za_zz_h1x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_h1x2 {
    #[inline]
    pub fn usmop4s_za_zz_h1x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod umop4s_za_zz_h1x2 {
    #[inline]
    pub fn umop4s_za_zz_h1x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod smop4a_za_zz_h2x1 {
    #[inline]
    pub fn smop4a_za_zz_h2x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_h2x1 {
    #[inline]
    pub fn sumop4a_za_zz_h2x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_h2x1 {
    #[inline]
    pub fn usmop4a_za_zz_h2x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod umop4a_za_zz_h2x1 {
    #[inline]
    pub fn umop4a_za_zz_h2x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod smop4s_za_zz_h2x1 {
    #[inline]
    pub fn smop4s_za_zz_h2x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_h2x1 {
    #[inline]
    pub fn sumop4s_za_zz_h2x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_h2x1 {
    #[inline]
    pub fn usmop4s_za_zz_h2x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod umop4s_za_zz_h2x1 {
    #[inline]
    pub fn umop4s_za_zz_h2x1(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod smop4a_za_zz_h2x2 {
    #[inline]
    pub fn smop4a_za_zz_h2x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_h2x2 {
    #[inline]
    pub fn sumop4a_za_zz_h2x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_h2x2 {
    #[inline]
    pub fn usmop4a_za_zz_h2x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod umop4a_za_zz_h2x2 {
    #[inline]
    pub fn umop4a_za_zz_h2x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b001u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod smop4s_za_zz_h2x2 {
    #[inline]
    pub fn smop4s_za_zz_h2x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_h2x2 {
    #[inline]
    pub fn sumop4s_za_zz_h2x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_h2x2 {
    #[inline]
    pub fn usmop4s_za_zz_h2x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001110u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
pub mod umop4s_za_zz_h2x2 {
    #[inline]
    pub fn umop4s_za_zz_h2x2(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001111u32 << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Zm.into()) << 17u32
                | 0b0000000u32 << 10u32
                | u32::from(N.into()) << 9u32
                | u32::from(Zn.into()) << 6u32
                | 0b011u32 << 3u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
