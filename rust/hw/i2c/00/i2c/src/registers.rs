// Copyright 2025 HUST OpenAtom Open Source Club.
// SPDX-License-Identifier: GPL-2.0-or-later

use bilge::prelude::*;
use migration::{impl_vmstate_bitsized};
// use bits::bits;
// use migration::{impl_vmstate_bitsized, impl_vmstate_forward};

#[doc(alias = "offset")]
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, common::TryInto)]
pub enum RegisterOffset {
    #[doc(alias = "I2C_CTRL")]
    CTRL = 0x00,

    #[doc(alias = "I2C_STATUS")]
    STATUS = 0x04,

    #[doc(alias = "I2C_ADDR")]
    ADDR = 0x08,

    #[doc(alias = "I2C_DATA")]
    DATA = 0x0C,

    #[doc(alias = "I2C_PRESCALE")]
    PRESCALE = 0x10,
}

#[bitsize(32)]
#[derive(Clone, Copy, Default, DebugBits, FromBits)]
pub struct Ctrl {
    pub en    : bool,
    pub start : bool,
    pub stop  : bool,
    pub rw    : bool,
    _reserved : u28,
}

impl_vmstate_bitsized!(Ctrl);
// impl Ctrl {
// }

#[bitsize(32)]
#[derive(Clone, Copy, Default, DebugBits, FromBits)]
pub struct Status {
    pub busy  : bool,
    pub ack   : bool,
    pub done  : bool,
    _reserved : u29,
}

#[bitsize(32)]
#[derive(Clone, Copy, Default, DebugBits, FromBits)]
pub struct Addr {
    pub addr  : u7,
    _reserved : u25,
}

impl_vmstate_bitsized!(Status);
// impl Status {
// }

