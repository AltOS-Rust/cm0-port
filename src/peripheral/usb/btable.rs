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

use volatile::Volatile;

#[derive(Copy, Clone, Debug)]
pub struct BTABLE {
    base_addr: *const u32,  //Address for the btable register
    rx_addr: *const u32,  //rx address for an end point
    tx_addr: *const u32,  //tx address for an end point
    rx_count: u32,  //Amount of data in the rx buffer
    tx_count: u32,  //Amount of data in the tx buffer
}

/*
 * BTABLE is the buffer allocation table, it stores the addresses of all the base address that
 * the endpoints use and the amount of data stored at those addresse
 */

impl BTABLE {
    pub fn new(base_addr: *const u32,rx_addr: *const u32, tx_addr: *const u32, rx_count: u32, tx_count: u32) -> Self {
        BTABLE {
            base_addr:base_addr,
            rx_addr:rx_addr,
            tx_addr:tx_addr,
            rx_count:rx_count,
            tx_count:tx_count,
        }
    }
    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        0x50
    }

    unsafe fn addr(&self) -> Volatile<u32> {
        let addr = self.base_addr() as *const u8;
        Volatile::new(addr.offset(self.mem_offset() as isize) as *const u32)
    }

    //Basic get and set functions
    pub fn get_rx_addr(&self) -> *const u32 {
        self.rx_addr
    }

    pub fn get_tx_addr(&self) -> *const u32 {
        self.tx_addr
    }

    pub fn get_rx_count(&self) -> u32 {
        self.rx_count
    }

    pub fn get_tx_count(&self) -> u32 {
        self.tx_count
    }

    pub fn set_rx_count(&mut self, rx_count:u32) {
        self.rx_count = rx_count;
    }

    pub fn set_tx_count(&mut self,tx_count:u32) {
        self.tx_count = tx_count;
    }

}

