/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_za_zzi_s4xi {
    #[inline]
    pub fn fmla_za_zzi_s4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(S.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod fdot_za32_z8z8i_4xi {
    #[inline]
    pub fn fdot_za32_z8z8i_4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0001u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod svdot_za_zzi_s4xi {
    #[inline]
    pub fn svdot_za_zzi_s4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b01u32 << 5u32
                | u32::from(U.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod usvdot_za_zzi_s4xi {
    #[inline]
    pub fn usvdot_za_zzi_s4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b01u32 << 5u32
                | u32::from(U.into()) << 4u32
                | 0b1u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod sdot_za32_zzi_4xi {
    #[inline]
    pub fn sdot_za32_zzi_4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(U.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod fdot_za_zzi_4xi {
    #[inline]
    pub fn fdot_za_zzi_4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0001u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod bfdot_za_zzi_4xi {
    #[inline]
    pub fn bfdot_za_zzi_4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0011u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod sdot_za_zzi_s4xi {
    #[inline]
    pub fn sdot_za_zzi_s4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b01u32 << 5u32
                | u32::from(U.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod usdot_za_zzi_s4xi {
    #[inline]
    pub fn usdot_za_zzi_s4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b01u32 << 5u32
                | u32::from(U.into()) << 4u32
                | 0b1u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod fmls_za_zzi_s4xi {
    #[inline]
    pub fn fmls_za_zzi_s4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(S.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod uvdot_za_zzi_s4xi {
    #[inline]
    pub fn uvdot_za_zzi_s4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b01u32 << 5u32
                | u32::from(U.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod suvdot_za_zzi_s4xi {
    #[inline]
    pub fn suvdot_za_zzi_s4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b01u32 << 5u32
                | u32::from(U.into()) << 4u32
                | 0b1u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod udot_za32_zzi_4xi {
    #[inline]
    pub fn udot_za32_zzi_4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(U.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod udot_za_zzi_s4xi {
    #[inline]
    pub fn udot_za_zzi_s4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b01u32 << 5u32
                | u32::from(U.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod sudot_za_zzi_s4xi {
    #[inline]
    pub fn sudot_za_zzi_s4xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010101u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i2.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b01u32 << 5u32
                | u32::from(U.into()) << 4u32
                | 0b1u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
