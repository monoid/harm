/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod insr_z_v_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001101000011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "insr_z_v_";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zdn_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zdn_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vm_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Vm_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_size_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_size_WIDTH: u32 = 2u32;
    #[inline]
    pub const fn insr_z_v_(
        size: ::aarchmrs_types::BitValue<2>,
        Vm: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b110100001110u32 << 10u32
                | Vm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
