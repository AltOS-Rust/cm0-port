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

/// Defines the possible directions that data can be read from.
#[derive(Copy, Clone, Debug)]
pub enum DataDirection {
    /// Data is read from the peripheral.
    FromPeriph,
    /// Data is read from memory.
    FromMem,
}

/// Defines the possible transfer data sizes of the peripheral and memory.
#[derive(Copy, Clone, Debug)]
pub enum PeriphAndMemSize {
    /// Eight bits.
    Eight,
    /// Sixteen bits.
    Sixteen,
    /// Thirty-two bits.
    ThirtyTwo,
}

/// Defines the priority of the channel requests.
///
/// If two requests have the same software priority, the channel with the lowest
/// number will get priority versus the channel with the highest number.
/// For example, channel 2 gets priority over channel 4.
#[derive(Copy, Clone, Debug)]
pub enum ChannelPriorityLevel {
    /// Low Priority.
    Low,
    /// Medium Priority.
    Medium,
    /// High Priority.
    High,
    /// Very High Priority.
    VeryHigh,
}

#[derive(Copy, Clone, Debug)]
pub struct CCR(u32);

impl CCR {
    /* Bit 0 EN: Channel enable
     *  This bit is set and cleared by software.
     *  0: Channel disabled
     *  1: Channel enabled
    */
    pub fn enable_dma(&mut self, enable: bool) {
        self.0 &= !(CCR_EN);
        if enable {
            self.0 |= CCR_EN;
        }
    }

    /* Bit 1 TCIE: Transfer complete interrupt enable
     *  This bit is set and cleared by software.
     *  0: TC interrupt disabled
     *  1: TC interrupt enabled
    */
    pub fn enable_transmit_complete_interrupt(&mut self, enable: bool) {
        self.0  &= !(CCR_TCIE);
        if enable {
            self.0 |= CCR_TCIE;
        }
    }

    /* Bit 2 HTIE: Half transfer interrupt enable
     *  This bit is set and cleared by software.
     *  0: HT interrupt disabled
     *  1: HT interrupt enabled
    */
    pub fn enable_half_transfer_interrupt(&mut self, enable: bool) {
        self.0 &= !(CCR_HTIE);
        if enable {
            self.0 |= CCR_HTIE;
        }
    }

    /* Bit 3 TEIE: Transfer error interrupt enable
     *  This bit is set and cleared by software.
     *  0: TE interrupt disabled
     *  1: TE interrupt enabled
    */
    pub fn enable_transfer_error_interrupt(&mut self, enable: bool) {
        self.0 &= !(CCR_TEIE);
        if enable {
            self.0 |= CCR_TEIE;
        }
    }

    /* Bit 4 DIR: Data transfer direction
     *  This bit is set and cleared by software.
     *  0: Read from peripheral
     *  1: Read from memory
    */
    pub fn set_data_transfer_direction(&mut self, data_dir: DataDirection) {
        let mask = match data_dir {
            DataDirection::FromPeriph => 0,
            DataDirection::FromMem => CCR_DIR,
        };

        self.0 &= !(CCR_DIR);
        self.0 |= mask;
    }

    /* Bit 5 CIRC: Circular mode
     *  This bit is set and cleared by software.
     *  0: Circular mode disabled
     *  1: Circular mode enabled
    */
    pub fn enable_circular_mode(&mut self, enable: bool) {
        self.0 &= !(CCR_CIRC);
        if enable {
            self.0 |= CCR_CIRC;
        }
    }

    /* Bit 6 PINC: Peripheral increment mode
     *  This bit is set and cleared by software.
     *  0: Peripheral increment mode disabled
     *  1: Peripheral increment mode enabled
    */
    pub fn enable_peripheral_increment_mode(&mut self, enable: bool) {
        self.0 &= !(CCR_PINC);
        if enable {
            self.0 |= CCR_PINC;
        }
    }

    /* Bit 7 MINC: Memory increment mode
     *  This bit is set and cleared by software.
     *  0: Memory increment mode disabled
     *  1: Memory increment mode enabled
    */
    pub fn enable_memory_increment_mode(&mut self, enable: bool) {
        self.0 &= !(CCR_MINC);
        if enable {
            self.0 |= CCR_MINC;
        }
    }

    /* Bits 9:8 PSIZE[1:0]: Peripheral size
     *  These bits are set and cleared by software.
     *  00: 8-bits
     *  01: 16-bits
     *  10: 32-bits
     *  11: Reserved
    */
    pub fn set_peripheral_size(&mut self, periph_size: PeriphAndMemSize) {
        let mask = match periph_size {
            PeriphAndMemSize::Eight => 0,
            PeriphAndMemSize::Sixteen => CCR_PSIZE0,
            PeriphAndMemSize::ThirtyTwo => CCR_PSIZE1,
        };

        self.0 &= !(CCR_PSIZE0 | CCR_PSIZE1);
        self.0 |= mask;
    }

    /* Bits 11:10 MSIZE[1:0]: Memory size
     *  These bits are set and cleared by software.
     *  00: 8-bits
     *  01: 16-bits
     *  10: 32-bits
     *  11: Reserved
    */
    pub fn set_memory_size(&mut self, mem_size: PeriphAndMemSize) {
        let mask = match mem_size {
            PeriphAndMemSize::Eight => 0,
            PeriphAndMemSize::Sixteen => CCR_MSIZE0,
            PeriphAndMemSize::ThirtyTwo => CCR_MSIZE1,
        };

        self.0 &= !(CCR_MSIZE0 | CCR_MSIZE1);
        self.0 |= mask;
    }

    /* Bits 13:12 PL[1:0]: Channel priority level
     *  These bits are set and cleared by software.
     *  00: Low
     *  01: Medium
     *  10: High
     *  11: Very high
    */
    pub fn set_channel_priority(&mut self, chan_priority: ChannelPriorityLevel) {
        let mask = match chan_priority {
            ChannelPriorityLevel::Low => 0,
            ChannelPriorityLevel::Medium => CCR_PL0,
            ChannelPriorityLevel::High => CCR_PL1,
            ChannelPriorityLevel::VeryHigh => (CCR_PL0 | CCR_PL1),
        };

        self.0 &= !(CCR_PL0 | CCR_PL1);
        self.0 |= mask;
    }

    /* Bit 14 MEM2MEM: Memory to memory mode
     *  This bit is set and cleared by software.
     *  0: Memory to memory mode disabled
     *  1: Memory to memory mode enabled
    */
    pub fn enable_mem2mem_mode(&mut self, enable: bool) {
        self.0 &= !(CCR_MEM2MEM);
        if enable {
            self.0 |= CCR_MEM2MEM;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ccr_enable_disable_dma() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.enable_dma(true);
        assert_eq!(ccr.0, 0b1);

        ccr.enable_dma(false);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_enable_transmit_complete_interrupt() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.enable_transmit_complete_interrupt(true);
        assert_eq!(ccr.0, 0b1 << 1);

        ccr.enable_transmit_complete_interrupt(false);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_enable_half_transfer_interrupt() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.enable_half_transfer_interrupt(true);
        assert_eq!(ccr.0, 0b1 << 2);

        ccr.enable_half_transfer_interrupt(false);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_enable_transfer_error_interrupt() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.enable_transfer_error_interrupt(true);
        assert_eq!(ccr.0, 0b1 << 3);

        ccr.enable_transfer_error_interrupt(false);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_set_data_transfer_direction() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.set_data_transfer_direction(DataDirection::FromMem);
        assert_eq!(ccr.0, 1 << 4);

        ccr.set_data_transfer_direction(DataDirection::FromPeriph);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_enable_circular_mode() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.enable_circular_mode(true);
        assert_eq!(ccr.0, 0b1 << 5);

        ccr.enable_circular_mode(false);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_enable_peripheral_increment_mode() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.enable_peripheral_increment_mode(true);
        assert_eq!(ccr.0, 0b1 << 6);

        ccr.enable_peripheral_increment_mode(false);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_enable_memory_increment_mode() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.enable_memory_increment_mode(true);
        assert_eq!(ccr.0, 0b1 << 7);

        ccr.enable_memory_increment_mode(false);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_set_peripheral_size() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.set_peripheral_size(PeriphAndMemSize::Sixteen);
        assert_eq!(ccr.0, 0b1 << 8);

        ccr.set_peripheral_size(PeriphAndMemSize::ThirtyTwo);
        assert_eq!(ccr.0, 0b1 << 9);

        ccr.set_peripheral_size(PeriphAndMemSize::Reserved);
        assert_eq!(ccr.0, 0b11 << 8);

        ccr.set_peripheral_size(PeriphAndMemSize::Eight);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_set_memory_size() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.set_memory_size(PeriphAndMemSize::Sixteen);
        assert_eq!(ccr.0, 0b1 << 10);

        ccr.set_memory_size(PeriphAndMemSize::ThirtyTwo);
        assert_eq!(ccr.0, 0b1 << 11);

        ccr.set_memory_size(PeriphAndMemSize::Reserved);
        assert_eq!(ccr.0, 0b11 << 10);

        ccr.set_memory_size(PeriphAndMemSize::Eight);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_set_channel_priority() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.set_channel_priority(ChannelPriorityLevel::Medium);
        assert_eq!(ccr.0, 0b1 << 12);

        ccr.set_channel_priority(ChannelPriorityLevel::High);
        assert_eq!(ccr.0, 0b1 << 13);

        ccr.set_channel_priority(ChannelPriorityLevel::VeryHigh);
        assert_eq!(ccr.0, 0b11 << 12);

        ccr.set_channel_priority(ChannelPriorityLevel::Low);
        assert_eq!(ccr.0, 0b0);
    }

    #[test]
    fn test_ccr_enable_mem2mem_mode() {
        let mut ccr = CCR(0);
        assert_eq!(ccr.0, 0b0);

        ccr.enable_mem2mem_mode(true);
        assert_eq!(ccr.0, 1 << 14);

        ccr.enable_mem2mem_mode(false);
        assert_eq!(ccr.0, 0b0);
    }
}
