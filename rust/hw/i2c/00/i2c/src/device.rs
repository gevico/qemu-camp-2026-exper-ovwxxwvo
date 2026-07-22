// SPDX-License-Identifier: GPL-2.0-or-later

// use std::{ffi::CStr, mem::size_of, ops::Add};
// use std::{ffi::CStr, mem::size_of,};
use std::{ffi::CStr, };
use system::prelude::*;
use common::prelude::*;
use util::prelude::*;
use qom::prelude::*;
use bql::prelude::*;
use hwcore::{prelude::*, IRQState};
use crate::registers::{self, Addr, Ctrl, RegisterOffset, Status};

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
pub struct I2CRegisters {
    pub ctrl     : registers::Ctrl,
    pub status   : registers::Status,
    pub addr     : registers::Addr,
    pub data     : u32,
    pub prescale : u32,
    }

#[repr(C)]
#[derive(qom::Object, hwcore::Device)]
/// I2C Device Model in QEMU
pub struct I2CState {
    pub parent_obj: ParentField<SysBusDevice>,
    pub iomem: MemoryRegion,
    pub regs: BqlRefCell<I2CRegisters>,

    // #[doc(alias = "clk")]
    // pub clock: Owned<Clock>,
    // #[doc(alias = "migrate_clk")]
    // #[property(rename = "migrate-clk", default = true)]
    // pub migrate_clock: bool,
    }

// static_assert!(size_of::<I2CState>() <= size_of::<crate::bindings::I2CState>());
// static_assert!(size_of::<I2CState>() <= size_of::<crate::bindings::SysBusDevice>());
// static_assert!(size_of::<I2CState>() <= size_of::<SysBusDevice>());
qom_isa!(I2CState : SysBusDevice, DeviceState, Object);

#[repr(C)]
pub struct I2CClass {
    parent_class: <SysBusDevice as ObjectType>::Class,
    /// The byte string that identifies the device.
    device_id: DeviceId,
    }

trait I2CImpl: SysBusDeviceImpl + IsA<I2CState> {
    const DEVICE_ID: DeviceId;
    }

impl I2CClass {
    fn class_init<T: I2CImpl>(&mut self) {
        self.device_id = T::DEVICE_ID;
        self.parent_class.class_init::<T>();
    }
    }

unsafe impl ObjectType for I2CState {
    type Class = I2CClass;
    const TYPE_NAME: &'static CStr = crate::TYPE_I2C;
    }

impl I2CImpl for I2CState {
    // const DEVICE_ID: DeviceId = DeviceId(&[0x11, 0x10, 0x14, 0x00, 0x0d, 0xf0, 0x05, 0xb1]);
    const DEVICE_ID: DeviceId = DeviceId(&[0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10]);
    }

impl ObjectImpl for I2CState {
    type ParentType = SysBusDevice;
    const INSTANCE_INIT: Option<unsafe fn(ParentInit<Self>)> = Some(Self::init);
    const INSTANCE_POST_INIT: Option<fn(&Self)> = Some(Self::post_init);
    const CLASS_INIT: fn(&mut Self::Class) = Self::Class::class_init::<Self>;
    }

impl DeviceImpl for I2CState {
    // const VMSTATE: Option<VMStateDescription<Self>> = Some(VMSTATE_I2C);
    const REALIZE: Option<fn(&Self) -> util::Result<()>> = Some(Self::realize);
    }

impl ResettablePhasesImpl for I2CState {
    const HOLD: Option<fn(&Self, ResetType)> = Some(Self::reset_hold);
    }

impl SysBusDeviceImpl for I2CState {}

impl I2CRegisters {
    pub(self) fn read(&mut self, offset: RegisterOffset) -> (bool, u32) {
        use RegisterOffset::*;
        // let mut update = false;
        let update = false;
        let result = match offset {
            CTRL     => u32::from(self.ctrl),
            STATUS   => u32::from(self.status),
            ADDR     => u32::from(self.addr),
            DATA     => self.data,
            PRESCALE => self.prescale,
        };
        (update, result)
    }

    pub(self) fn write(&mut self, offset: RegisterOffset, value: u32, _device: &I2CState) -> bool {
        use RegisterOffset::*;
        match offset {
            CTRL     => self.ctrl     = Ctrl::from(value),
            STATUS   => self.status   = Status::from(value),
            ADDR     => self.addr     = Addr::from(value),
            DATA     => self.data     = value,
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

impl I2CState {
    unsafe fn init(mut this: ParentInit<Self>) {
        static I2C_OPS: MemoryRegionOps<I2CState> = MemoryRegionOpsBuilder::<I2CState>::new()
            .read(&I2CState::read)
            .write(&I2CState::write)
            .little_endian()
            .impl_sizes(4, 4)
            .build();

        // SAFETY: this and this.iomem are guaranteed to be valid at this point
        MemoryRegion::init_io(
            &mut uninit_field_mut!(*this, iomem),
            &I2C_OPS,
            "i2c",
            0x1000,
        );

        uninit_field_mut!(*this, regs).write(Default::default());
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
            Ok(field) => {
                let (update_irq, result) = self.regs.borrow_mut().read(field);
                // trace::trace_i2c_read(offset, result, c"");
                if update_irq {
                    self.update();
                }
                result.into()
            }
        }
    }

    fn write(&self, offset: hwaddr, value: u64, _size: u32) {
        let mut update_irq = false;
        if let Ok(field) = RegisterOffset::try_from(offset) {
            // trace::trace_i2c_write(offset, value as u32, c"");
            update_irq = self.regs.borrow_mut().write(field, value as u32, self);
        } else {
            log_mask_ln!(
                Log::GuestError,
                "I2CState::write: Bad offset {offset} value {value}"
            );
        }
        if update_irq {
            self.update();
        }
    }

    fn realize(&self) -> util::Result<()> {
        Ok(())
    }

    fn reset_hold(&self, _type: ResetType) {
        self.regs.borrow_mut().reset();
    }

    fn update(&self) {
        // let regs = self.regs.borrow();
        // let flags = regs.int_level & regs.int_enabled;
        // trace::trace_i2c_irq_state(flags != 0);
        // for (irq, i) in self.interrupts.iter().zip(IRQMASK) {
        //     irq.set(flags.any_set(i));
        // }
    }

    pub fn post_load(&self, _version_id: u8) -> Result<(), migration::InvalidError> {
        // self.regs.borrow_mut().post_load()
        Ok(())
    }
}

#[no_mangle]
pub unsafe extern "C" fn i2c_create(
    addr: u64,
    irq: *mut IRQState,
) -> *mut DeviceState {
    // SAFETY: The callers promise that they have owned references.
    // They do not gift them to pl011_create, so use `Owned::from`.
    let irq = unsafe { Owned::<IRQState>::from(&*irq) };

    let dev = I2CState::new();
    dev.sysbus_realize().unwrap_fatal();
    dev.mmio_map(0, addr);
    dev.connect_irq(0, &irq);

    // The pointer is kept alive by the QOM tree; drop the owned ref
    dev.as_mut_ptr()
}

