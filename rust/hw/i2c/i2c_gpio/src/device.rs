// SPDX-License-Identifier: GPL-2.0-or-later

use std::{ffi::CStr, mem::size_of};
use common::prelude::*;
use system::prelude::*;
use util::prelude::*;
use qom::prelude::*;
use bql::prelude::*;
use hwcore::{prelude::*};
use crate::registers::{self, RegisterOffset, Ctrl, Status, Addr};
use i2c_core::core::{I2CBus};
use i2c_slave::at24c02::AT24C02Slave;

#[derive(Clone, Copy)]
struct DeviceId(&'static [u8; 8]);
impl std::ops::Index<hwaddr> for DeviceId {
    type Output = u8;
    fn index(&self, idx: hwaddr) -> &Self::Output {
        &self.0[idx as usize]
    }
    }

#[repr(C)]
#[derive(Debug, Default)]
pub struct I2CGPIORegisters {
    pub ctrl     : registers::Ctrl,
    pub status   : registers::Status,
    pub addr     : registers::Addr,
    pub data     : u32,
    pub prescale : u32,
    }

#[repr(C)]
#[derive(qom::Object, hwcore::Device)]
/// I2C Device Model in QEMU
pub struct I2CGPIOState {
    pub parent_obj: ParentField<SysBusDevice>,
    pub iomem: MemoryRegion,
    pub regs: BqlRefCell<I2CGPIORegisters>,
    pub i2c_bus: BqlRefCell<I2CBus>,
    // at24c02_slave: BqlRefCell<Box<AT24C02Slave>>,
    }

static_assert!(size_of::<I2CGPIOState>() <= size_of::<crate::bindings::I2CGPIOState>());
qom_isa!(I2CGPIOState : SysBusDevice, DeviceState, Object);

#[repr(C)]
pub struct I2CGPIOClass {
    parent_class: <SysBusDevice as ObjectType>::Class,
    /// The byte string that identifies the device.
    device_id: DeviceId,
    }

trait I2CGPIOImpl: SysBusDeviceImpl + IsA<I2CGPIOState> {
    const DEVICE_ID: DeviceId;
    }

impl I2CGPIOClass {
    fn class_init<T: I2CGPIOImpl>(&mut self) {
        self.device_id = T::DEVICE_ID;
        self.parent_class.class_init::<T>();
    }
    }

unsafe impl ObjectType for I2CGPIOState {
    type Class = I2CGPIOClass;
    const TYPE_NAME: &'static CStr = crate::TYPE_I2C_GPIO;
    }

impl I2CGPIOImpl for I2CGPIOState {
    // const DEVICE_ID: DeviceId = DeviceId(&[0x11, 0x10, 0x14, 0x00, 0x0d, 0xf0, 0x05, 0xb1]);
    const DEVICE_ID: DeviceId = DeviceId(&[0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10]);
    }

impl ObjectImpl for I2CGPIOState {
    type ParentType = SysBusDevice;
    const INSTANCE_INIT: Option<unsafe fn(ParentInit<Self>)> = Some(Self::init);
    const INSTANCE_POST_INIT: Option<fn(&Self)> = Some(Self::post_init);
    const CLASS_INIT: fn(&mut Self::Class) = Self::Class::class_init::<Self>;
    }

impl DeviceImpl for I2CGPIOState {
    // const VMSTATE: Option<VMStateDescription<Self>> = Some(VMSTATE_I2C);
    const REALIZE: Option<fn(&Self) -> util::Result<()>> = Some(Self::realize);
    }

impl ResettablePhasesImpl for I2CGPIOState {
    const HOLD: Option<fn(&Self, ResetType)> = Some(Self::reset_hold);
    }

impl SysBusDeviceImpl for I2CGPIOState {}

impl I2CGPIORegisters {
    pub(self) fn read(&mut self, offset: RegisterOffset) -> u32 {
        use RegisterOffset::*;
        let result = match offset {
            CTRL     => u32::from(self.ctrl),
            STATUS   => u32::from(self.status),
            ADDR     => u32::from(self.addr),
            DATA     => self.data,
            PRESCALE => self.prescale,
        };
        result
    }

    pub(self) fn write(&mut self, offset: RegisterOffset, value: u32, device: &I2CGPIOState) -> bool {
        use RegisterOffset::*;
        match offset {
            ADDR     => self.addr     = Addr::from(value),
            DATA     => self.data     = value,
            CTRL     => {
                let mut i2c_bus = device.i2c_bus.borrow_mut();
                let ctrl = Ctrl::from(value);
                match (ctrl.en(), ctrl.start(), ctrl.stop(), ctrl.rw()) {
                    (true, true, false, _) => {
                        let addr = u32::from(self.addr) as u8;
                        let ret  = i2c_bus.start_transfer(addr, ctrl.rw());
                        // self.status.set_busy(true);
                        self.status.set_busy(i2c_bus.is_busy());
                        self.status.set_ack(ret == 0);
                        self.status.set_done(true);
                    },
                    (true, false, true, false) => {
                        i2c_bus.end_transfer();
                        // self.status.set_busy(false);
                        self.status.set_busy(i2c_bus.is_busy());
                        self.status.set_done(true);
                    },
                    (true, false, false, false) => {
                        let data = self.data as u8;
                        let ret  = i2c_bus.send(data);
                        self.status.set_busy(i2c_bus.is_busy());
                        self.status.set_ack(ret == 0);
                        self.status.set_done(true);
                    },
                    (true, false, false, true) => {
                        let data = i2c_bus.recv();
                        self.data = data as u32;
                        self.status.set_busy(i2c_bus.is_busy());
                        self.status.set_done(true);
                    },
                    _ => {},
                };
                self.ctrl = ctrl
            },
            STATUS   => self.status   = Status::from(value),
            PRESCALE => self.prescale = value,
        }
        false
    }

    pub fn reset(&mut self) {
        self.ctrl     = Ctrl::default();
        self.status   = Status::default();
        self.addr     = Addr::default();
        self.data     = 0;
        self.prescale = 0;
    }
    }

impl I2CGPIOState {
    unsafe fn init(mut this: ParentInit<Self>) {
        static I2CGPIO_OPS: MemoryRegionOps<I2CGPIOState> = MemoryRegionOpsBuilder::<I2CGPIOState>::new()
            .read(&I2CGPIOState::read)
            .write(&I2CGPIOState::write)
            .little_endian()
            .impl_sizes(4, 4)
            .build();

        // SAFETY: this and this.iomem are guaranteed to be valid at this point
        MemoryRegion::init_io(
            &mut uninit_field_mut!(*this, iomem),
            &I2CGPIO_OPS,
            "i2c",
            0x1000,
        );

        uninit_field_mut!(*this, regs).write(Default::default());
        uninit_field_mut!(*this, i2c_bus).write(BqlRefCell::new(I2CBus::new()));
        // uninit_field_mut!(*this, at24c02_slave).write(BqlRefCell::new(Box::new(AT24C02Slave::new(0x50))));
    }

    fn post_init(&self) {
        self.init_mmio(&self.iomem);
        // for irq in self.interrupts.iter() {
        //     self.init_irq(irq);
        // }
    }

    fn read(&self, offset: hwaddr, _size: u32) -> u64 {
        match RegisterOffset::try_from(offset) {
            Err(v) if (0x03f8..0x0400).contains(&(v >> 2)) => {
                let device_id = self.get_class().device_id;
                u64::from(device_id[(offset - 0x0fe0) >> 2])
            }
            Err(_) => {
                log_mask_ln!(Log::GuestError, "I2CState::read: Bad offset {offset}");
                0
            }
            Ok(reg) => {
                let result = self.regs.borrow_mut().read(reg);
                result.into()
            }
        }
    }

    fn write(&self, offset: hwaddr, value: u64, _size: u32) {
        if let Ok(reg) = RegisterOffset::try_from(offset) {
            self.regs.borrow_mut().write(reg, value as u32, self);
            // self.bus.borrow().start_transfer(reg., is_recv)
        } else {
            log_mask_ln!(
                Log::GuestError,
                "I2CState::write: Bad offset {offset} value {value}"
            );
        }
    }

    fn realize(&self) -> util::Result<()> {
        self.i2c_bus.borrow_mut().attach(Box::new(AT24C02Slave::new(0x50)));
        Ok(())
    }

    fn reset_hold(&self, _type: ResetType) {
        self.regs.borrow_mut().reset();
    }

    pub fn post_load(&self, _version_id: u8) -> Result<(), migration::InvalidError> {
        // self.regs.borrow_mut().post_load()
        Ok(())
    }
}

#[no_mangle]
pub unsafe extern "C" fn i2c_gpio_create(
    addr: u64,
    // irq: *mut IRQState,
) -> *mut DeviceState {
    // SAFETY: The callers promise that they have owned references.
    // They do not gift them to pl011_create, so use `Owned::from`.
    // let irq = unsafe { Owned::<IRQState>::from(&*irq) };

    let dev = I2CGPIOState::new();
    dev.sysbus_realize().unwrap_fatal();
    dev.mmio_map(0, addr);
    // dev.connect_irq(0, &irq);

    // The pointer is kept alive by the QOM tree; drop the owned ref
    dev.as_mut_ptr()
}

