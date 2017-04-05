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
use volatile::Volatile;

/*
 * TODO:find right values for Isochronous
 * But I don't think I really need to care about that
 * It is still setup to be added in so its easier in the future
 * when someone who cares comes around
 * Bulk type is broken up into Input and Output type
*/

#[derive(Copy,Clone,PartialEq, Debug)]
pub enum USBType {
    Control,
    Isochronous,
    Input,
    Output,
    Interrupt,
}

#[derive(Copy,Clone, Debug)]
pub struct EP {
    base_addr: *const u32,
    offset:u32,
    buffer_addr: *mut u32,
    usb_type:USBType,
    buffer_max: u32,

    //Marked when a packet hasn't been received yet
    pending:bool,
    //Marked when we don't need to send a packet
    flushed:bool,
}

/* EP contain an address to a buffer within the SRAM and registers to be used by the USB to know
 * what hardware functions it can perform, the EPs also carry buffers that is filled by the host
 * and is grabbed by the host to send/recv data between the device
 */

impl EP {
  pub const fn new(base_addr:*const u32,offset:u32,buffer_addr: *mut u32,buffer_max:u32,usb_type:USBType) -> Self {
        EP {
                base_addr: base_addr,
                offset:offset,
                buffer_addr: buffer_addr,
                usb_type:usb_type,
                buffer_max:buffer_max,
                pending:false,
                flushed:false,
          }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    //We determine the offset needed for the EP here for each specific register
    fn mem_offset(&self) -> u32 {
        match self.offset {
            0 => 0x00,
            1 => 0x04,
            2 => 0x08,
            3 => 0x0C,
            4 => 0x10,
            5 => 0x14,
            6 => 0x18,
            7 => 0x1C,
            _ => panic!("EP::mem_offset - offset was not a valid value "),
        }
    }

    unsafe fn addr(&self) -> Volatile<u32> {
        let addr = self.base_addr() as *const u8;
        Volatile::new(addr.offset(self.mem_offset() as isize) as *const u32)
    }


    //public get and sets
    pub fn set_buffer(&mut self,buffer_location: *mut u32) {
        self.buffer_addr = buffer_location;
    }

    pub fn get_buffer_addr(&self) -> *mut u32 {
        self.buffer_addr
    }

    pub fn set_flushed(&mut self, flushed:bool) {
        self.flushed = flushed;
    }

    pub fn get_flushed(&self) -> bool {
        self.flushed
    }

    pub fn set_pending(&mut self, pending:bool) {
        self.pending = pending;
    }

    pub fn get_pending(&self) -> bool {
        self.pending
    }

    fn set_toggle(&self, curr_value:u32,mask:u32,desired_value:u32) -> u32 {
        (curr_value ^ desired_value) & mask
    }

    /*
     * This uses the EP's USBType value to determine how tho initialize itself (which bits to flip to
     * determine what kind of end point it is, it calls a helper function init_ep (below) and uses
     * constants found in defs.rs
     */

    pub fn init(&self) {
        match self.usb_type {
            USBType::Control => self.init_ep(USB_EPR_CONTROL,USB_EPR_TYPE_CONTROL,USB_EPR_STAT_RX_VALID,USB_EPR_STAT_TX_NAK),
            USBType::Interrupt => self.init_ep(USB_EPR_INT,USB_EPR_TYPE_INTERRUPT,USB_EPR_STAT_RX_DISABLED,USB_EPR_STAT_TX_NAK),
            USBType::Input => self.init_ep(USB_EPR_INPUT,USB_EPR_TYPE_BULK,USB_EPR_STAT_RX_DISABLED,USB_EPR_STAT_TX_NAK),
            USBType::Output => self.init_ep(USB_EPR_OUTPUT,USB_EPR_TYPE_BULK,USB_EPR_STAT_RX_VALID,USB_EPR_STAT_TX_DISABLED),
            _ => panic!("EP::init - USBType not currently supported!"),
        }
    }

    /*
     * This is the actual function that flips the correct bit to determine what type of end point it
     */

    fn init_ep(&self,addr:u32, con_type:u32,stat_rx:u32,stat_tx:u32) {
        unsafe {
            let mut reg = self.addr();
            let temp = (self.base_addr().offset(self.mem_offset() as isize)) as u32;

            reg.store((0 << USB_EPR_CTR_RX) |
                                (temp & ( 1 << USB_EPR_DTOG_RX)) |
                                self.set_toggle(temp,
                                (USB_EPR_STAT_RX_MASK << USB_EPR_STAT_RX),
                                (stat_rx << USB_EPR_STAT_RX)) |
                                (con_type << USB_EPR_EP_TYPE) |
                                (0 << USB_EPR_EP_KIND) |
                                (0 << USB_EPR_CTR_TX) |
                                (temp & (1 << USB_EPR_DTOG_TX)) |
                                self.set_toggle(temp,
                                (USB_EPR_STAT_TX_MASK << USB_EPR_STAT_TX),
                                (stat_tx << USB_EPR_STAT_TX)) |
                                (addr << USB_EPR_EA));
        }
    }

    pub fn is_busy(&self) -> bool {
        unsafe {
            let reg = self.base_addr().offset(self.mem_offset() as isize) as u32;

            if (self.usb_type == USBType::Output) || (self.usb_type == USBType::Control) {
                let epr = (reg >> USB_EPR_STAT_TX) & USB_EPR_STAT_TX_MASK;
                return epr == USB_EPR_STAT_TX_VALID;
            }

            if self.usb_type == USBType::Input {
                let epr = (reg >> USB_EPR_STAT_RX) & USB_EPR_STAT_RX_MASK;
                return epr == USB_EPR_STAT_RX_VALID;
            }

            panic!("EP::is_busy - asked an invalid endpoint if they are busy!");
        }
    }

    pub fn clear_rx_tx(&mut self) {
        self.preserve_mask();
        self.invariant();
        unsafe {
              let mut reg = self.addr();
              let temp  = self.base_addr().offset(self.mem_offset() as isize) as u32;
            reg.store(!(1 << USB_EPR_CTR_RX) & temp);
            reg.store(!(1 << USB_EPR_CTR_TX) & temp);
        }
    }

    fn preserve_mask(&mut self) {
        unsafe {
            let mut reg = self.addr();
            let temp  = self.base_addr().offset(self.mem_offset() as isize) as u32;
            reg.store(USB_EPR_PRESERVE_MASK & temp);
        }
    }

    fn invariant(&mut self) {
        unsafe {
            let mut reg = self.addr();
            let temp  = self.base_addr().offset(self.mem_offset() as isize) as u32;
            reg.store(USB_EPR_INVARIANT | temp);
        }
    }

    pub fn clear_ctr_bits(&mut self) {
        unsafe {
            let mut reg = self.addr();
            self.preserve_mask();
            self.invariant();
            let temp = self.base_addr().offset(self.mem_offset() as isize) as u32;
            reg.store(!(1 << USB_EPR_CTR_RX) & temp);
            reg.store(!(1 << USB_EPR_CTR_TX) & temp);
        }
    }

    pub fn set_stat(&mut self,stat_tx:u32,stat_rx:u32) {

        unsafe {
            let premask  = self.base_addr().offset(self.mem_offset() as isize) as u32;

            self.preserve_mask();
            self.invariant();

            let mut reg = self.addr();
            let temp  = self.base_addr().offset(self.mem_offset() as isize) as u32;

            if (self.usb_type == USBType::Output) || (self.usb_type == USBType::Control) {
                reg.store(self.set_toggle(premask,USB_EPR_STAT_TX_MASK << USB_EPR_STAT_TX,
                                          stat_tx << USB_EPR_STAT_TX) | temp);
                return;
            }

            if self.usb_type == USBType::Input {
                reg.store(self.set_toggle(premask,USB_EPR_STAT_RX_MASK << USB_EPR_STAT_RX,
                                          stat_rx << USB_EPR_STAT_RX) | temp);
                return;
            }

            if self.usb_type == USBType::Interrupt {
                reg.store(self.set_toggle(premask,USB_EPR_STAT_RX_MASK << USB_EPR_STAT_RX,
                                          stat_rx << USB_EPR_STAT_RX) | temp);
                return;
            }

            panic!("EP::set_stat - tried to set the stat of an invalid endpoint!");
        }
    }

    pub fn epr_ctr_rx(&self) -> bool {
        unsafe {
            let reg = self.base_addr().offset(self.mem_offset() as isize) as u32;
            let temp = (reg >> USB_EPR_CTR_RX) & 1;
            temp == 1
        }
    }

    pub fn epr_ctr_tx(&self) -> bool {
          unsafe {
            let reg  = self.base_addr().offset(self.mem_offset() as isize) as u32;
            let temp = (reg >> USB_EPR_CTR_TX) & 1;
            temp == 1
        }
    }

    pub fn epr_setup(&self) -> bool {
          unsafe {
            let reg = self.base_addr().offset(self.mem_offset() as isize) as u32;
            let temp = (reg >> USB_EPR_SETUP) & 1;
            temp == 1
        }
    }

    //Set the value of the buffer addr to the value of data shifted by 24 bits
    pub fn ep_set(&mut self,data: *const u8) {
        unsafe {
            *self.buffer_addr = (*data as u32) << 24;
        }
    }

    /*
      * send data copies data from the temp buffer of the usb module into the buffer of a given EP
      * this is done by shifting u8 bits accordingly to fit into size u32. Because of this it grabs 4
      * bytes at a time to fit it into a buffer
      */

    pub fn send_data(&mut self, source:[u8;BUF_SIZE], bytes:u32) {
        if bytes > 64 {
            panic!("EP::send_data - Attempting to write more than 64 bytes to the buffer");
        }
        unsafe{
            let mut b:usize = (bytes - 1) as usize; //Max index of Bytes to copy
            let mut s:usize = 0;
            while b > 3 {
                *self.buffer_addr = (source[s + 3] as u32) << 24
                                  | (source[s + 2] as u32) << 16
                                  | (source[s + 1] as u32) << 8
                                  | (source[s] as u32);
                //Move the buffer pointer forward
                self.buffer_addr = self.buffer_addr.offset(1) as *mut u32;
                s += 4;
                b -= 4;
            }
            match b {
                3 => {
                  *self.buffer_addr =(source[s + 2] as u32) << 16
                                    | (source[s + 1] as u32) << 8
                                    | (source[s] as u32);
                },
                2 => {
                    *self.buffer_addr = (source[s + 1] as u32) << 8
                                      | (source[s] as u32);
                },
                1 => {
                    *self.buffer_addr = source[s] as u32;
                },
                _ => {}, //Do nothing when 0 is left over
            }
        }
    }

    /*
      * receive_data moves the data from an EP buffer into the temp buffer of the usb module
      * It splits its single u32 chunk of data into 4 chunks and places it into the temp buffer u32 ->
      * u8
      */

    pub fn receive_data(&mut self, destination:&mut [u8;BUF_SIZE], bytes:u32) {
        if bytes > 64 {
            panic!("EP::send_data - Attempting to write more than 64 bytes to the buffer");
        }
        unsafe{
            let mut s:usize = 0;
            let mut d:isize = 0;
            let max:usize = (bytes - 4) as usize;
            let mut curr = *self.buffer_addr.offset(d);
            while s < max {
                destination[s] = (curr) as u8;
                destination[s + 1] = (curr >> 8) as u8;
                destination[s + 2] = (curr >> 16) as u8;
                destination[s + 3] = (curr >> 24) as u8;
                d -= 1;
                s += 4;
                //Move the buffer pointer backwards then grab the value from the new location
                self.buffer_addr = self.buffer_addr.offset(d);
                curr = *self.buffer_addr;
            }
            match s - max {
                3 => {
                    destination[s] = (curr) as u8;
                    destination[s + 1] = (curr >> 8) as u8;
                    destination[s + 2] = (curr >> 16) as u8;
                },
                2 => {
                    destination[s] = (curr) as u8;
                    destination[s + 1] = (curr >> 8) as u8;
                },
                1 => {
                    destination[s] = (curr) as u8;
                },
                _ => {}, //Do nothing when 0 is left over
            }
        }
    }
}

/*
 * Testing Strategy
 * Create a Test.rs file to hold the set up information (See test.rs in altos-core/src)
 * Test that registers get set properly
 * Test that data is written/coped out of buffers correctly
 * Full buffer
 * Full flag gets set
 * Buffer Underflow
 * Buffer is too large
 * No data
 * Test masking returns expected values
 * Test that sram is allocating space in the correct intervals
 * System panic conditions go off (that can)
 * Regiser value is reset to the correct value
 */
