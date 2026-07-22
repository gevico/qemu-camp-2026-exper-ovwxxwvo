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

#ifndef HW_AT24C02_H
#define HW_AT24C02_H

#include "hw/core/sysbus.h"
#include "qom/object.h"

#define TYPE_AT24C02 "at24c02"
OBJECT_DECLARE_SIMPLE_TYPE(AT24C02State, AT24C02)

struct AT24C02State {
    SysBusDevice parent_obj;
    // MemoryRegion iomem;
    /*
     * Since some users embed this struct directly, we must
     * ensure that the C struct is at least as big as the Rust one.
     */
    uint8_t padding_for_rust[512];
};

DeviceState *at24c02e_create(hwaddr addr);

#endif

