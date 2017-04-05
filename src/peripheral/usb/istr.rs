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

use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct ISTR {
    base_addr: *const u32,
}

/*
 * ISTR is the register that contains the status of all the interrupt sources so
 * I can see which events caused an interrupt request (which endpoint to use etc)
 */

impl Register for ISTR {
    fn new(base_addr: *const u32) -> Self {
        ISTR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        0x44
    }
}

impl ISTR {
    pub fn set(&self,port:u32) {
        unsafe {
            let mut reg = self.addr();
            reg.store(port);
        }
    }
    pub fn get(&self) -> *const u32 {
        unsafe {
            self.addr().as_ptr()
        }
    }

    //Gets the current interrupt flag set from the hardware
    pub fn get_interrupt(&self) -> usize {
        unsafe {
            let temp = *self.base_addr().offset(self.mem_offset() as isize);
            let mut reg = self.addr();
            reg.store(temp);


            if (temp & (1 << USB_ISTR_CTR)) != 0 {
                return (temp & USB_ISTR_EP_ID_MASK) as usize;
            }

            if (temp & (1 << USB_ISTR_RESET)) != 0 {
                return ISTR_RESET_FLAG; //Arbitrarily defined constant in defs.rs for usb module to react to
            }

            if (temp & (1 << USB_ISTR_SUSP)) != 0 {
                return ISTR_SUSPEND_FLAG; //Arbitrarily defined constant in defs.rs for usb module to react to
            }

            if (temp & (1 << USB_ISTR_WKUP)) != 0 {
                return ISTR_WAKEUP_FLAG; //Arbitrarily defined constant in defs.rs for usb module to react to
            }
        }
        panic!("istr::get_interrupt - interrupt not found!")
    }

    pub fn clear(&self) {
        unsafe {
            let mut reg = self.addr();
            reg.store(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_istr_clear() {
        let istr = test::create_register::<ISTR>();
        istr.clear();
        assert_eq!(istr.register_value(), 0);
    }
}
