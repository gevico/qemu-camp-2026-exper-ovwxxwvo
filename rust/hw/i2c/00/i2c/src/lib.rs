// Copyright 2025 HUST OpenAtom Open Source Club.
// Author(s): Chen Miao <chenmiao@openatom.club>
// Author(s): Chao Liu <chao.liu@openatom.club>
// SPDX-License-Identifier: GPL-2.0-or-later

//! I2C bus model for G233 SoC (QEMU Camp 2026 Rust experiment).
//!
//! This module provides a pure-Rust I2C bus abstraction modeled after
//! the upstream QEMU C I2C infrastructure (`include/hw/i2c/i2c.h`).
//!
//! Students must implement the TODO-marked methods to make the unit
//! tests pass. The design follows the upstream pattern:
//!
//! - [`I2CEvent`]: bus state change events (START_RECV, START_SEND, FINISH, NACK)
//! - [`I2CSlave`]: trait for I2C slave devices (send, recv, event)
//! - [`I2CBus`]: bus that manages slave devices and routes transfers

// pub mod bus;
// pub use crate::{I2CBus, I2CEvent, I2CSlave};
pub mod bindings;
pub mod core;
pub mod device;
pub mod registers;
mod test;

pub const TYPE_I2C: &::std::ffi::CStr = c"i2c";

