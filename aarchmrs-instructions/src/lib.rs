/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

#![no_std]
#![allow(
    non_snake_case,
    non_camel_case_types,
    clippy::identity_op,
    clippy::too_many_arguments,
    clippy::module_inception
)]
#[cfg(feature = "A32")]
pub mod A32;
#[cfg(feature = "A64")]
pub mod A64;
#[cfg(feature = "T32")]
pub mod T32;
