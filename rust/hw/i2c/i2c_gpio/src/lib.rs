// Copyright 2025 HUST OpenAtom Open Source Club.
// Author(s): Chen Miao <chenmiao@openatom.club>
// Author(s): Chao Liu <chao.liu@openatom.club>
// SPDX-License-Identifier: GPL-2.0-or-later

//! I2C-GPIO QEMU Device Model
//!
//! This library implements a device model for the PrimeCell® I2C-GPIO
//! device in QEMU.
//!
//! # Library crate
//!
//! See [`I2C-GPIOState`](crate::device::I2C-GPIOState) for the device model type and
//! the [`registers`] module for register types.


pub mod bindings;
pub mod device;
pub mod registers;

pub use device::i2c_gpio_create;

pub const TYPE_I2C_GPIO: &::std::ffi::CStr = c"i2c-gpio";


