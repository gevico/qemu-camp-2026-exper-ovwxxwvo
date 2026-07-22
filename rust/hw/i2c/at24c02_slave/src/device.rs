// SPDX-License-Identifier: GPL-2.0-or-later

use std::{ffi::CStr, mem::size_of};
use common::prelude::*;
use system::prelude::*;
use util::prelude::*;
use qom::prelude::*;
use bql::prelude::*;
use hwcore::{prelude::*};
use i2c_core::core::{I2CBus, I2CSlave, I2CEvent};
use i2c_gpio::device::I2CGPIOState;

#[derive(Clone, Copy)]
struct DeviceId(&'static [u8; 8]);
impl std::ops::Index<hwaddr> for DeviceId {
    type Output = u8;
    fn index(&self, idx: hwaddr) -> &Self::Output {
        &self.0[idx as usize]
    }
    }

#[repr(C)]
struct AT24C02Slave {
    addr: u8,
    regs: [u8; 256],
    pointer: u8,
    first_byte: bool,
    }

#[repr(C)]
#[derive(qom::Object, hwcore::Device)]
pub struct AT24C02State {
    pub parent_obj: ParentField<SysBusDevice>,
    i2c_slave: BqlRefCell<AT24C02Slave>,
    i2c_bus: *mut BqlRefCell<I2CBus>,
    // pub dev_addr: u8,
    }

static_assert!(size_of::<AT24C02State>() <= size_of::<crate::bindings::AT24C02State>());
qom_isa!(AT24C02State : SysBusDevice, DeviceState, Object);

#[repr(C)]
pub struct AT24C02Class {
    parent_class: <SysBusDevice as ObjectType>::Class,
    /// The byte string that identifies the device.
    device_id: DeviceId,
    }

trait AT24C02Impl: SysBusDeviceImpl + IsA<AT24C02State> {
    const DEVICE_ID: DeviceId;
    }

impl AT24C02Class {
    fn class_init<T: AT24C02Impl>(&mut self) {
        self.device_id = T::DEVICE_ID;
        self.parent_class.class_init::<T>();
    }
    }

unsafe impl ObjectType for AT24C02State {
    type Class = AT24C02Class;
    const TYPE_NAME: &'static CStr = crate::TYPE_AT24C02;
    }

impl AT24C02Impl for AT24C02State {
    // const DEVICE_ID: DeviceId = DeviceId(&[0x11, 0x10, 0x14, 0x00, 0x0d, 0xf0, 0x05, 0xb1]);
    const DEVICE_ID: DeviceId = DeviceId(&[0x11, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10]);
    }

impl ObjectImpl for AT24C02State {
    type ParentType = SysBusDevice;
    const INSTANCE_INIT: Option<unsafe fn(ParentInit<Self>)> = Some(Self::init);
    const INSTANCE_POST_INIT: Option<fn(&Self)> = Some(Self::post_init);
    const CLASS_INIT: fn(&mut Self::Class) = Self::Class::class_init::<Self>;
    }

impl DeviceImpl for AT24C02State {
    // const VMSTATE: Option<VMStateDescription<Self>> = Some(VMSTATE_I2C);
    const REALIZE: Option<fn(&Self) -> util::Result<()>> = Some(Self::realize);
    }

impl ResettablePhasesImpl for AT24C02State {
    const HOLD: Option<fn(&Self, ResetType)> = Some(Self::reset_hold);
    }

impl SysBusDeviceImpl for AT24C02State {}

impl AT24C02Slave {
    fn new(addr: u8) -> Self {
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
        if self.first_byte {
            self.pointer = data;
            self.first_byte = false;
        } else {
            self.regs[self.pointer as usize] = data;
            self.pointer = self.pointer.wrapping_add(1);
        }
        0
    }
    }

impl AT24C02State {
    unsafe fn init(mut this: ParentInit<Self>) {
        // static I2C_OPS: MemoryRegionOps<AT24C02State> = MemoryRegionOpsBuilder::<AT24C02State>::new()
        //     .read(&AT24C02State::read)
        //     .write(&AT24C02State::write)
        //     .little_endian()
        //     .impl_sizes(4, 4)
        //     .build();

        // uninit_field_mut!(*this, regs).write(Default::default());
        uninit_field_mut!(*this, i2c_slave).write(BqlRefCell::new(AT24C02Slave::new(0)));
        uninit_field_mut!(*this, i2c_bus).write(std::ptr::null_mut());
    }

    // fn slave_init(&self, addr: u8) {
    fn slave_init(&mut self, parent: *mut DeviceState, addr: u8) {
        let mut i2c_slave = self.i2c_slave.borrow_mut();
        *i2c_slave = AT24C02Slave::new(addr);

        unsafe {
            let i2c_gpio = &mut *(parent as *mut I2CGPIOState);
            self.i2c_bus = &mut i2c_gpio.i2c_bus as *mut _;
        }
    }

    fn post_init(&self) {
        // self.init_mmio(&self.iomem);
    }

    fn realize(&self) -> util::Result<()> {
        // let parent = self.parent_obj.borrow();
        // let i2c = parent.downcast::<I2CGPIOState>().ok_or(util::Error::NotFound)?;
        // let
        // let mut bus = i2c.i2c_bus.borrow_mut();
        // let slave = self.i2c_slave.borrow();
        // bus.attach(Box::new(slave));
        Ok(())
    }

    fn reset_hold(&self, _type: ResetType) {
        // self.regs.borrow_mut().reset();
    }

    pub fn post_load(&self, _version_id: u8) -> Result<(), migration::InvalidError> {
        Ok(())
    }
}

#[no_mangle]
pub unsafe extern "C" fn at24c02e_create(
    // addr: u8,
    parent: *mut DeviceState ,addr: u8,
) -> *mut DeviceState {
    // SAFETY: The callers promise that they have owned references.

    let dev = AT24C02State::new();
    dev.slave_init(parent, addr);
    dev.sysbus_realize().unwrap_fatal();

    // The pointer is kept alive by the QOM tree; drop the owned ref
    dev.as_mut_ptr()
}

