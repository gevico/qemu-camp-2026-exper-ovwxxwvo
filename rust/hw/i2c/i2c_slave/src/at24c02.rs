// Copyright 2025 HUST OpenAtom Open Source Club.
// Author(s): Chen Miao <chenmiao@openatom.club>
// Author(s): Chao Liu <chao.liu@openatom.club>
// SPDX-License-Identifier: GPL-2.0-or-later

use i2c_core::core::{I2CSlave, I2CEvent};

pub struct AT24C02Slave {
    pub addr: u8,
    pub regs: [u8; 256],
    pub pointer: u8,
    pub first_byte: bool,
}

impl AT24C02Slave {
    pub fn new(addr: u8) -> Self {
        Self {
            addr,
            regs: [0xFF; 256],
            pointer: 0,
            first_byte: true,
        }
    }
}

impl I2CSlave for AT24C02Slave {
    fn address(&self) -> u8 {
        self.addr
    }

    fn event(&mut self, event: I2CEvent) -> i32 {
        if event == I2CEvent::StartSend {
            self.first_byte = true;
        }
        0
    }

    fn recv(&mut self) -> u8 {
        let val = self.regs[self.pointer as usize];
        self.pointer = self.pointer.wrapping_add(1);
        val
    }

    fn send(&mut self, data: u8) -> i32 {
        const PAGE_MASK: u8 = 0x07;
        if self.first_byte {
            self.pointer = data;
            self.first_byte = false;
        } else {
            let offset = self.pointer & PAGE_MASK;
            self.regs[self.pointer as usize] = data;
            if offset < 7 {
                self.pointer = self.pointer + 1;
            } else {
                self.pointer = self.pointer & !PAGE_MASK;
            }
        }
        0
    }
}

