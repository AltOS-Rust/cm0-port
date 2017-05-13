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

mod ccr;
mod cndtr;
mod cpar;
mod cmar;
mod defs;
mod ifcr;

use peripheral::{rcc};
use peripheral::usart::{Usart, UsartX, DMAMode};
use peripheral::usart::defs::{USART2_ADDR, TDR_OFFSET};
use io::{TX_BUFFER, RX_BUFFER};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use volatile::Volatile;
use self::ccr::CCR;
use self::cndtr::CNDTR;
use self::cpar::CPAR;
use self::cmar::CMAR;
use self::ifcr::IFCR;
use self::defs::*;
use self::ccr::{DataDirection, PeriphAndMemSize, ChannelPriorityLevel};

impl Index<DMAChannel> for [DMAChannelRegs] {
    type Output = DMAChannelRegs;

    fn index(&self, chan: DMAChannel) -> &Self::Output {
        &self[chan as usize]
    }
}

impl IndexMut<DMAChannel> for [DMAChannelRegs] {
    fn index_mut(&mut self, chan: DMAChannel) -> &mut Self::Output {
        &mut self[chan as usize]
    }
}

impl Index<DMAChannel> for RawDMA {
    type Output = DMAChannelRegs;

    fn index(&self, chan: DMAChannel) -> &Self::Output {
        &self.channel[chan as usize]
    }
}

impl IndexMut<DMAChannel> for RawDMA {
    fn index_mut(&mut self, chan: DMAChannel) -> &mut Self::Output {
        &mut self.channel[chan as usize]
    }
}
/// Defines the availabe DMA Channels for STM32F04.
///
/// Used as C-like enum in order to index into array of DMAChannelRegs.
#[derive(Copy, Clone, Debug)]
pub enum DMAChannel {
    /// DMA Channel 1 (Index 0)
    One,
    /// DMA Channel 2 (Index 1)
    Two,
    /// DMA Channel 3 (Index 2)
    Three,
    /// DMA Channel 4 (Index 3)
    Four,
    /// DMA Channel 5 (Index 4)
    Five,
}

#[derive(Copy, Clone, Debug)]
#[doc(hidden)]
pub struct DMAChannelRegs {
    ccr: CCR,
    cndtr: CNDTR,
    cpar: CPAR,
    cmar: CMAR,
    _res: u32,
}

impl DMAChannelRegs {
    /// Enable the DMA.
    pub fn enable_dma(&mut self) {
        self.ccr.enable_dma(true);
    }

    /// Disable the DMA.
    pub fn disable_dma(&mut self) {
        self.ccr.enable_dma(false);
    }

    /// Enable TC interrupt. This interrupt occurs at the end of the transfer.
    pub fn enable_transmit_complete_interrupt(&mut self) {
        self.ccr.enable_transmit_complete_interrupt(true);
    }

    /// Disable TC interrupt. This interrupt occurs at the end of the transfer.
    pub fn disable_transmit_complete_interrupt(&mut self) {
        self.ccr.enable_transmit_complete_interrupt(false);
    }

    /// Enable HT interrupt. This interrupt occurs when half of the bytes are transferred.
    pub fn enable_half_transfer_interrupt(&mut self) {
        self.ccr.enable_half_transfer_interrupt(true);
    }

    /// Disable HT interrupt. This interrupt occurs when half of the bytes are transferred.
    pub fn disable_half_transfer_interrupt(&mut self) {
        self.ccr.enable_half_transfer_interrupt(false);
    }

    /// Enable TE interrupt.
    ///
    /// This interrupt occurs when an error is generated through a read or write access.
    /// If a transfer error is generated, the faulty channel is disabled through a
    /// hardware clear of the EN bit in the corresponding Channel configuration
    /// register (DMA_CCRx).
    pub fn enable_transfer_error_interrupt(&mut self) {
        self.ccr.enable_transfer_error_interrupt(true);
    }

    /// Disable TE interrupt.
    ///
    /// This interrupt occurs when an error is generated through a read or write access.
    /// If a transfer error is generated, the faulty channel is disabled through a
    /// hardware clear of the EN bit in the corresponding Channel configuration
    /// register (DMA_CCRx).
    pub fn disable_transfer_error_interrupt(&mut self) {
        self.ccr.enable_transfer_error_interrupt(false);
    }

    /// Set the transfer direction to either read from memory or read from the peripheral.
    ///
    /// This interrupt occurs when an error is generated through a read or write access.
    /// If a transfer error is generated, the faulty channel is disabled through a
    /// hardware clear of the EN bit in the corresponding Channel configuration
    /// register (DMA_CCRx).
    pub fn set_data_transfer_direction(&mut self, data_dir: DataDirection) {
        self.ccr.set_data_transfer_direction(data_dir);
    }

    /// Enable circular mode.
    ///
    /// When enabled, the number of data to be transferred is automaticaly reloaded
    /// with the initial value programmed during the channel configuration phase,
    /// and the DMA requests continue to be served.
    pub fn enable_circular_mode(&mut self) {
        self.ccr.enable_circular_mode(true);
    }

    /// Disable circular mode.
    ///
    /// When enabled, the number of data to be transferred is automaticaly reloaded
    /// with the initial value programmed during the channel configuration phase,
    /// and the DMA requests continue to be served.
    pub fn disable_circular_mode(&mut self) {
        self.ccr.enable_circular_mode(false);
    }

    /* The address of the next transfer will be the previous one incremented by 1, 2, or 4
     * depending on the chosen data size.
     *
     * The first transfer address is the one programmed in the DMA_CPAR/DMA_CMAR registers.
     * During transfer operations, these registers keep the initally programmed value.
     * The current transfer addresses (in the current internal peripheral/memory address
     * register) are not accessible by software.
     *
     * Note: If the channel is configured in non-circular mode, no DMA request is served
     * after the last transfer (once the number of items to be transferred has reached zero).
     * In order to reload a new number of data items to be transferred into the DMA_CNDTRx
     * register, the DMA channel must be disabled.
     */
    /// Enable peripheral increment mode.
    ///
    /// When enabled automatically post-increments the peripheral pointer
    /// after each transaction.
    pub fn enable_peripheral_increment_mode(&mut self) {
        self.ccr.enable_peripheral_increment_mode(true);
    }

    /// Disable peripheral increment mode.
    ///
    /// When enabled automatically post-increments the peripheral pointer
    /// after each transaction.
    pub fn disable_peripheral_increment_mode(&mut self) {
        self.ccr.enable_peripheral_increment_mode(false);
    }

    /// Enable memory increment mode.
    ///
    /// When enabled automatically post-increments the memory pointer
    /// after each transaction.
    pub fn enable_memory_increment_mode(&mut self) {
        self.ccr.enable_memory_increment_mode(true);
    }

    /// Disable memory increment mode.
    ///
    /// When enabled automatically post-increments the memory pointer
    /// after each transaction.
    pub fn disable_memory_increment_mode(&mut self) {
        self.ccr.enable_memory_increment_mode(false);
    }

    /// Sets the peripheral data size.
    pub fn set_peripheral_size(&mut self, periph_size: PeriphAndMemSize) {
        self.ccr.set_peripheral_size(periph_size);
    }

    /// Sets the memory data size.
    pub fn set_memory_size(&mut self, mem_size: PeriphAndMemSize) {
        self.ccr.set_memory_size(mem_size);
    }

    /// Sets the channel priority.
    ///
    /// If two channels have the same priority, the lowest number channel will
    /// have priority over the higher number channel.
    pub fn set_channel_priority(&mut self, chan_priority: ChannelPriorityLevel) {
        self.ccr.set_channel_priority(chan_priority);
    }

    /// Enable memory-to-memory transfers.
    ///
    /// When enabled, the DMA channels can work without being triggered by a request
    /// from a peripheral. The transfer stops once teh DMA_CNDTRx register reaches
    /// zero. Memory-to-memory mode cannot be used at the same time as circular mode.
    pub fn enable_mem2mem_mode(&mut self) {
        self.ccr.enable_mem2mem_mode(true);
    }

    /// Disable memory-to-memory transfers.
    ///
    /// When enabled, the DMA channels can work without being triggered by a request
    /// from a peripheral. The transfer stops once teh DMA_CNDTRx register reaches
    /// zero. Memory-to-memory mode cannot be used at the same time as circular mode.
    pub fn disable_mem2mem_mode(&mut self) {
        self.ccr.enable_mem2mem_mode(false);
    }

    /// Set the number of data to be transferred. Up to 65535.
    pub fn set_number_of_data(&mut self, num_data: u16) {
        self.cndtr.set_ndt(num_data);
    }

    /// Set the peripheral address.
    ///
    /// This is the base address of the peripheral that is using the DMA.
    /// The data will be moved from/to this address to/from the memory after
    /// the peripheral event.
    pub fn set_peripheral_address(&mut self, periph_addr: *const u32) {
        self.cpar.set_pa(periph_addr);
    }

    /// Set the memory address.
    ///
    /// This is the memory address where the data will be written to or read from
    /// after the peripheral event.
    pub fn set_memory_address(&mut self, mem_addr: *const u32) {
        self.cmar.set_ma(mem_addr);
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[doc(hidden)]
pub struct RawDMA {
    isr: u32,
    ifcr: IFCR,
    channel: [DMAChannelRegs; 5]
}

/// The DMA peripheral is used to provide high-speed data transfer between peripherals
/// and memory as well as memory to memory. This struct is used to configure the DMA,
/// manage DMA channels, and handle DMA interrupts.
#[derive(Copy, Clone, Debug)]
pub struct DMA(Volatile<RawDMA>);

impl DMA {
    /// Creates a new DMA object to configure the specifications for the
    /// DMA peripheral.
    pub fn new() -> Self {
        unsafe {
            DMA(Volatile::new(DMA_ADDR as *const _))
        }
    }
}

impl Deref for DMA {
    type Target = RawDMA;

    fn deref(&self) -> &Self::Target {
        &*(self.0)
    }
}

impl DerefMut for DMA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *(self.0)
    }
}

impl RawDMA {
    /// Clear all DMA interrupt flags.
    pub fn channel_global_interrupt_clear(&mut self, chan: DMAChannel) {
        self.ifcr.channel_global_interrupt_clear(chan);
    }

    /// Clear the TC flag. The TC flag is set when the transfer of data has completed.
    pub fn channel_transfer_complete_clear(&mut self, chan: DMAChannel) {
        self.ifcr.channel_transfer_complete_clear(chan);
    }

    /// Clear the HTC flag. The HTC flag is set when half the data to be
    /// transfered has completed.
    pub fn channel_half_transfer_clear(&mut self, chan: DMAChannel) {
        self.ifcr.channel_half_transfer_clear(chan);
    }

    /// Clear the TE flag.
    ///
    /// This interrupt occurs when an error is generated through a read or write access.
    /// If a transfer error is generated, the faulty channel is disabled through a
    /// hardware clear of the EN bit in the corresponding Channel configuration
    /// register (DMA_CCRx).
    pub fn channel_transfer_error_clear(&mut self, chan: DMAChannel) {
        self.ifcr.channel_transfer_error_clear(chan);
    }

}

/// Initialize the DMA peripheral
///
/// Set the clock for the DMA and makes the necessary calls in order to configure
/// peripherals intended for use with the DMA.
pub fn init() {
    let mut rcc = rcc::rcc();
    rcc.enable_peripheral(rcc::Peripheral::DMA);

    init_usart(DMAChannel::Four, DMAChannel::Five);
}

fn init_usart(usart_tx: DMAChannel, usart_rx: DMAChannel) {
    let mut dma = DMA::new();

    // Enable DMA bits on the Usart2 peripheral.
    let mut usart2 = Usart::new(UsartX::Usart2);
    usart2.set_dma_mode(DMAMode::All);

    dma[usart_tx].disable_dma();
    unsafe {
        dma[usart_tx].set_peripheral_address(USART2_ADDR.offset(TDR_OFFSET as isize));
    }
    dma[usart_tx].set_memory_address(&TX_BUFFER);
}




// Channel configuration procedure
// The following sequence should be followed to configure a DMA channel x (where x is the
// channel number).
// 1.Set the peripheral register address in the DMA_CPARx register. The data will be
// moved from/ to this address to/ from the memory after the peripheral event.
// 2. Set the memory address in the DMA_CMARx register. The data will be written to or
// read from this memory after the peripheral event.
// 3. Configure the total number of data to be transferred in the DMA_CNDTRx register.
// After each peripheral event, this value will be decremented.
// 4. Configure the channel priority using the PL[1:0] bits in the DMA_CCRx register
// 5. Configure data transfer direction, circular mode, peripheral & memory incremented
// mode, peripheral & memory data size, and interrupt after half and/or full transfer in the
// DMA_CCRx register
// 6. Activate the channel by setting the ENABLE bit in the DMA_CCRx register.
//
// For code example refer to the Appendix section A.5.1: DMA Channel Configuration
// sequence code example.
//
// As soon as the channel is enabled, it can serve any DMA request from the peripheral
// connected on the channel.
//
// Once half of the bytes are transferred, the half-transfer flag (HTIF) is set and an interrupt is
// generated if the Half-Transfer Interrupt Enable bit (HTIE) is set. At the end of the transfer,
// the Transfer Complete Flag (TCIF) is set and an interrupt is generated if the Transfer
// Complete Interrupt Enable bit (TCIE) is set.
