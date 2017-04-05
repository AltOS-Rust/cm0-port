/*
 * copyright (c) 2017 altos-rust team
 *
 * this program is free software: you can redistribute it and/or modify
 * it under the terms of the gnu general public license as published by
 * the free software foundation, either version 3 of the license, or
 * (at your option) any later version.
 *
 * this program is distributed in the hope that it will be useful,
 * but without any warranty; without even the implied warranty of
 * merchantability or fitness for a particular purpose.  see the
 * gnu general public license for more details.
 *
 * you should have received a copy of the gnu general public license
 * along with this program. if not, see <http://www.gnu.org/licenses/>.
 */

use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct BCDR {
    base_addr: *const u32,
}

/*
 * BCDR is the battery charging something register, I just use it so we can signal to the host
 * disconnect/connect of the USB hardware
 */

impl Register for BCDR {
    fn new(base_addr: *const u32) -> Self {
        BCDR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        0x58
    }
}

impl BCDR {
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

    pub fn disable_usb_pullup(&self) {
        unsafe{
            let mut reg = self.addr();
            let mask = *reg.as_ptr();
            reg.store((0 << USB_BCDR_DPPU) & mask);
        }
    }

    pub fn enable_usb_pullup(&self) {
        unsafe{
            let mut reg = self.addr();
            let mask = *reg.as_ptr();
            reg.store((1 << USB_BCDR_DPPU) | mask);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_bcdr_enable() {
        let bcdr = test::create_register::<BCDR>();
        bcdr.enable_usb_pullup();
        assert_eq!(bcdr.register_value(), 1 << USB_BCDR_DPPU);
    }

    #[test]
    fn test_bcdr_disable() {
        let bcdr = test::create_register::<BCDR>();
        bcdr.disable_usb_pullup();
        assert_eq!(bcdr.register_value(), 0 << USB_BCDR_DPPU);
    }
}
