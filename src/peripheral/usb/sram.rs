/*
 * Copyright (C) 2017 AltOS-Rust Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::defs::*;

pub enum USBSize {
    Control,
    Interrupt,
    Bulk,
}

#[derive(Copy,Clone, Debug)]
pub struct SRAM {
    base_addr: *const u32,
    position_pointer: u32,
}

/*
 * SRAM simply just grabs from the board's SRAM location and supplies a pointer to an address
 * Then it moves the pointer by the "allocated" amount (very lazy allocation, cannot "reallocate
 * memory"
 */

impl SRAM {
    pub fn new(base_addr: *const u32) -> Self {
        SRAM {
            base_addr:base_addr,
            position_pointer:0,
        }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    pub fn allocate_btable_space(&mut self) -> *const u32 {
        unsafe {
            let current_pointer = self.base_addr.offset(self.position_pointer as isize) as *const u32;
            self.position_pointer += 8 * BDT_SIZE;

            current_pointer
        }
    }

    /*
     * Function returns current address, the pushes the
     * position_pointer farther by a predetermined size
     * This supplies end points with buffer space
     */

    pub fn allocate_usb_space(&mut self, usb_size:USBSize) -> *mut u32 {
        unsafe{
            let current_pointer = self.base_addr.offset(self.position_pointer as isize) as *mut u32;
            //Space has been given to an endpoint so move the buffer pointer

            self.position_pointer += self.get_size(usb_size);
            current_pointer
        }
    }

    //TODO: change the struct into a mapping of these values so we don't need this helper function
    fn get_size(&self,usb_size:USBSize) -> u32{
        match usb_size {
            USBSize::Control => 32,
            USBSize::Interrupt => 8,
            USBSize::Bulk => 64,
        }
    }
}
