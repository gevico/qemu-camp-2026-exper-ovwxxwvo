/*
 * This program is free software; you can redistribute it and/or modify it
 * under the terms and conditions of the GNU General Public License,
 * version 2 or later, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef HW_I2C_GPIO_H
#define HW_I2C_GPIO_H

#include "hw/core/sysbus.h"
#include "qom/object.h"

#define TYPE_I2C_GPIO "i2c-gpio"
OBJECT_DECLARE_SIMPLE_TYPE(I2CGPIOState, I2C_GPIO)

struct I2CGPIOState {
    SysBusDevice parent_obj;
    MemoryRegion iomem;
    /*
     * Since some users embed this struct directly, we must
     * ensure that the C struct is at least as big as the Rust one.
     */
    uint8_t padding_for_rust[32];
};

// DeviceState *i2c_gpio_create(hwaddr addr, qemu_irq irq);
DeviceState *i2c_gpio_create(hwaddr addr);

#endif

