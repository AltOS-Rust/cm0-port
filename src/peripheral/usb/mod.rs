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

use super::{Control, Register};
use volatile::Volatile;
use arm::asm;
use peripheral::rcc;
use interrupt::{nvic,Priority};
use peripheral::syscfg;
use interrupt;

use self::defs::*;

mod defs;

//Registers used by USB
mod cntr;
mod istr;
mod bcdr;

mod btable;
mod sram;
mod epnr;

pub fn usb() -> USB {
    USB::usb()
}

pub struct USB {
    mem_addr: *const u32,
    cntr: cntr::CNTR,
    istr: istr::ISTR,
    bcdr: bcdr::BCDR,
    btable: [btable::BTABLE; EP_NUM],
    ep: [ep::EP;EP_NUM],
    sram: sram::SRAM,
    //Temp buffers
    rx_buffer: [u8;BUF_SIZE], // Host -> Temp Buffer -> rx_buffer -> USB  //USB Recieving Data
    tx_buffer: [u8;BUF_SIZE],  // USB -> tx_buffer -> Temp Buffer -> Host  //USB Sending Data
    rx_count: u32, //We can use this value to see how much data exist in each buffer
    tx_count: u32, //Same as above
    ep0_receive: u32,
    usb_setup: u8, //This is filled by by the host during start up
}

impl Control for USB {
    unsafe fn mem_addr(&self) -> Volatile<u32> {
        Volatile::new(self.mem_addr)
    }
}

//Size per EP location
const USB_SIZE: [u32;EP_NUM] = [32,8,64,64];

impl USB {

    fn usb() -> Self {
        //Assign the memory locations here
        // NOTE: This address is reserved space??
        let mut sram = sram::SRAM::new(SRAM_ADDR);

        //Initialize the end points and grab space from memory using sram
        let ep0_rx_addr = sram.allocate_usb_space(sram::USBSize::Control);
        let ep0_tx_addr = sram.allocate_usb_space(sram::USBSize::Control);

        let ep: [ep::EP;EP_NUM] = [ep::EP::new(USB_ADDR,0,ep0_rx_addr,32,ep::USBType::Control),
                                   ep::EP::new(USB_ADDR,1,sram.allocate_usb_space(sram::USBSize::Interrupt),8,ep::USBType::Interrupt),
                                   ep::EP::new(USB_ADDR,2,sram.allocate_usb_space(sram::USBSize::Bulk),64,ep::USBType::Output),
                                   ep::EP::new(USB_ADDR,3,sram.allocate_usb_space(sram::USBSize::Bulk),64,ep::USBType::Input)];

        //initializing the btable to save the addresses of the endpoint buffers
        let btable: [btable::BTABLE;EP_NUM] =
                [btable::BTABLE::new(USB_ADDR,ep0_rx_addr,ep0_tx_addr,RX_COUNT_SIZE,0),
                 btable::BTABLE::new(USB_ADDR,ep[1].get_buffer_addr(),ep[1].get_buffer_addr(),0,0),
                 btable::BTABLE::new(USB_ADDR,ep[2].get_buffer_addr(),ep[2].get_buffer_addr(),RX_COUNT_SIZE,0),
                 btable::BTABLE::new(USB_ADDR,ep[3].get_buffer_addr(),ep[3].get_buffer_addr(),0,0)];

        let usb = USB {
            mem_addr: USB_ADDR,
            cntr: cntr::CNTR::new(USB_ADDR),
            istr: istr::ISTR::new(USB_ADDR),
            bcdr: bcdr::BCDR::new(USB_ADDR),
            btable: btable,
            sram: sram,
            ep: ep,
            rx_buffer: [0;BUF_SIZE],
            tx_buffer: [0;BUF_SIZE],
            rx_count: 0,
            tx_count: 0,
            ep0_receive: 0,
            usb_setup: 0,
        };

        usb
    }

    // usb_init starts up the USB hardware by first performing a remap of the PA11/PA12
    // Then selecting the HSI48 clock using the RCC module and enabling the USB hardware
    // via the RCC, it calls usb_enable as a helper function
    pub fn usb_init(&self) {
        let rcc = rcc::rcc();
        let syscfg = syscfg::syscfg();

        rcc.enable_peripheral(rcc::Peripheral::SysCfgComp);
        syscfg.cfgr_remap_bits(syscfg::RemapConf::PA11PA12);

        self.usb_enable(rcc);
        self.ep_init();
    }

    pub fn usb_disable(&self) {
        let rcc = rcc::rcc();

        unsafe {
            asm::disable_interrupts();
        }

        self.cntr.reset_controller();
        self.istr.clear();
        self.bcdr.disable_usb_pullup();
        self.cntr.turn_off_device();

        rcc.disable_peripheral(rcc::Peripheral::USB);

        unsafe {
            asm::enable_interrupts();
        }
    }

    fn ep_init(&self) {
        unsafe{
            asm::disable_interrupts();
            for i in 0..EP_NUM {
                self.ep[i].init();
            }
            asm::enable_interrupts();
        }
    }

    fn btable_init(&mut self) {
        self.btable[EP0].set_rx_count(0);
        self.btable[EP0].set_tx_count(0);
    }

    fn usb_enable(&self,rcc:rcc::RCC) {

        let nvic = nvic();

        //NOTE: need cfgr3 from rcc ask danny about that later
        //if this doesn't work
        rcc.set_pll_source(rcc::Clock::HSI48);
        rcc.enable_peripheral(rcc::Peripheral::USB);

        self.bcdr.enable_usb_pullup();
        //clear reset condition
        //NOTE: no way to do this via rcc, implement later or ask danny about it
        self.bcdr.disable_usb_pullup();

        unsafe {
            asm::disable_interrupts();
        }

        // Routing interrupts using nvic
        nvic.enable_interrupt(interrupt::Hardware::USB);
        nvic.set_priority(Priority::High,interrupt::Hardware::USB);

        self.cntr.reset_controller();
        self.cntr.clear();
        self.istr.clear();
        self.cntr.enable_interrupts();

        unsafe {
            asm::enable_interrupts();
            for _ in 0..1000 {
                asm::nop();
            }
        }

        self.bcdr.enable_usb_pullup();
    }

    fn usb_send_data(&mut self,ep_num:usize) {

        /* //Figre out a way to do this with interrupst while adhearing to Rust crap
        let mut pending = self.ep[ep_num].get_pending();
        while pending {
            pending = self.ep[ep_num].get_pending();
            //sit and spin until someone tells us to stop pending
            //TODO: figure out how to yield in this scope
        }
        */

        self.ep[ep_num].set_pending(true);

        if self.tx_count != USB_SIZE[ep_num] {
            self.ep[ep_num].set_flushed(true);
        }

        //NOTE: copy out data
        self.ep[ep_num].send_data(self.tx_buffer,self.tx_count);
        unsafe {
            asm::disable_interrupts();
            self.ep[ep_num].set_stat(USB_EPR_STAT_TX_VALID,0);
            asm::enable_interrupts();
        }
    }

    fn usb_recv_data(&mut self,ep_num:usize) {

        //TODO: rethink this also finish this
        //Check to make sure you aren't doing too much here if not go for it
        self.ep[ep_num].receive_data(&mut self.rx_buffer,self.btable[ep_num].get_rx_count());
        //NOTE:copy in data
    }

    //Interrupt checking
    fn usb_isr(&mut self) {
        let interrupt = self.istr.get_interrupt();
        if interrupt < EP_NUM {

            //Save the SW write bits and clear CTR_RX/CTR/TX bits
            let temp = self.ep[interrupt];
            self.ep[interrupt].clear_rx_tx();


            match interrupt {
                EP0 =>{
                    if temp.epr_ctr_rx() {
                        if temp.epr_setup() {
                            self.ep0_receive |= USB_EP0_GOT_SETUP;
                        } else {
                            self.ep0_receive |= USB_EP0_GOT_RX_DATA;
                        }
                    }
                    if temp.epr_ctr_tx() {
                        self.ep0_receive |= USB_EP0_GOT_TX_ACK;
                    }

                    let handle:u32 = self.ep0_receive;
                    self.usb_ep0_handle(handle);
                },
                INT =>{
                    if temp.epr_ctr_tx() {
                        self.ep[interrupt].set_stat(0,USB_EPR_STAT_TX_NAK);
                    }
                },
                IN =>{
                    if temp.epr_ctr_tx(){
                        self.ep[interrupt].set_pending(true);
                        //wakeup(usb_pending)
                    }
                },
                OUT =>{
                    if temp.epr_ctr_rx() {
                          self.ep[interrupt].set_pending(true);
                        //wakeup(usb_pending)
                    }
                },
                _ => panic!("USB::usb_isr - ep not found!"),
            }
            //
        } //END interrupt < EP_NUM

        match interrupt {
            ISTR_RESET_FLAG =>{
                self.ep_init();
                self.btable_init();
            },
            ISTR_SUSPEND_FLAG =>{
                self.usb_suspend();
            },
            ISTR_WAKEUP_FLAG =>{
                self.usb_wakeup();
            },
            _ => panic!("USB::usb_isr - interrupt not found!"),
        }
    }

    fn usb_ep0_handle(&mut self,handle:u32) {
        self.ep0_receive = 0;

        if (handle & USB_EP0_GOT_SETUP) != 0 {
            self.usb_ep0_setup();
        }

        if (handle & USB_EP0_GOT_RX_DATA) != 0 {
            //TODO:fill ep0 rx buffer with data in
        }

        if (handle & USB_EP0_GOT_TX_ACK) != 0 {
            //TODO:set up address pending then flush buffer
        }
    }

    fn usb_ep0_setup(&mut self) {
        //Pull packet out the fifo
        //Set ep0 to look at it's tx buffer
        self.ep[EP0].set_buffer(self.btable[EP0].get_tx_addr() as *mut u32);
        self.ep[EP0].ep_set(&self.usb_setup);
        self.btable[EP0].set_tx_count(8);

        //Fill the ep0
        let mut len:u32 = self.btable[EP0].get_rx_count() & USB_BDT_COUNT_RX_COUNT_RX_MASK;

        //Pull the data out the packet
        self.ep[EP0].send_data(self.rx_buffer,len);

        //ACK the packet
        self.ep[EP0].set_stat(0,USB_EPR_STAT_RX_VALID);

        //TODO: The use the data in the ep0 tx buffer to determine usb_setup stuff
    }

    fn usb_suspend(&self) {
        self.cntr.suspend_device();
        self.cntr.low_power_mode();
    }

    fn usb_wakeup(&self) {
        self.cntr.resume_device();
    }

    fn usb_putchar(&mut self, c: char) {
        //We block interrupts and place the char into the tx buffer
        //If the tx buffer is full then we send it to ep input

            unsafe {
                asm::disable_interrupts();
                //Wait for usb in to be ready
                //Flush usb in

                self.tx_buffer[self.tx_count as usize] = c as u8;
                self.tx_count += 1;

                if self.tx_count == MAX_BUF_SIZE {
                    //putchar is full, we must flush

                }

                asm::enable_interrupts();
            }
    }

    fn usb_pollchar(&mut self) -> i8 {
        let mut c:i8 = 0;
        //if the usb isn't running we read again

        c = self.rx_buffer[self.rx_count as usize] as i8;
        self.rx_count -= 1;
        c
    }

    fn usb_getchar(&mut self) -> u8 {
        let mut c:u8 = 0;
        let mut temp:i8 = 0;
        unsafe {
            asm::disable_interrupts();
            while temp == READ_AGAIN {
                temp = self.usb_pollchar();
                //sleep
            }
            asm::enable_interrupts();
        }
        //return the polled char out
        c = temp as u8;
        c
    }


//Temp buffers for every endpoint in stmf code
// We want to do it in a smarter way
// * Allocate on the fly
// Send and Recieve take in endpoints and act on them
// usb_send( endPoint:endPoint ) { .. }
// usb_recieve( endPoin:endPoint ) { .. }
// Temp buffer == 64 buts of u8

// THINGS THAT MUST BE DONE SO WE CAN START TESTING
// * Send
// * Recieve
// * Put Char On Buffer
// * Take Char From Buffer

}
