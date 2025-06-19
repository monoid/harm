/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ssra_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ssra_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl ssra_z_zi_ {
        #[inline]
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b11100u32 << 11u32
                    | self.U.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
pub mod srsra_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct srsra_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl srsra_z_zi_ {
        #[inline]
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b11101u32 << 11u32
                    | self.U.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
pub mod usra_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usra_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl usra_z_zi_ {
        #[inline]
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b11100u32 << 11u32
                    | self.U.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
pub mod ursra_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ursra_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl ursra_z_zi_ {
        #[inline]
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b11101u32 << 11u32
                    | self.U.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
