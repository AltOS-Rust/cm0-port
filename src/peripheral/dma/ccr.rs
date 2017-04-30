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

#[derive(Copy, Clone, Debug)]
pub struct CCR(u32);

pub enum DataDirection {
    FromPeriph,
    FromMem,
}

pub enum PeriphAndMemSize {
    Eight,
    Sixteen,
    ThirtyTwo,
    Reserved,
}

pub enum ChannelPriorityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl CCR {
    pub fn enable_dma(&mut self, enable: bool) {
        self.0 &= !(CCR_EN);
        if enable {
            self.0 |= CCR_EN;
        }
    }

    pub fn enable_transmit_complete_interrupt(&mut self, enable: bool) {
        self.0  &= !(CCR_TCIE);
        if enable {
            self.0 |= CCR_TCIE;
        }
    }

    pub fn enable_half_transfer_interrupt(&mut self, enable: bool) {
        self.0 &= !(CCR_HTIE);
        if enable {
            self.0 |= CCR_HTIE;
        }
    }

    pub fn enable_transfer_error_interrupt(&mut self, enable: bool) {
        self.0 &= !(CCR_TEIE);
        if enable {
            self.0 |= CCR_TEIE;
        }
    }

    pub fn set_data_transfer_direction(&mut self, data_dir: DataDirection) {
        let mask = match data_dir {
            DataDirection::FromPeriph => !(CCR_DIR),
            DataDirection::FromMem => CCR_DIR,
        };

        self.0 &= mask;
    }

    pub fn enable_circular_mode(&mut self, enable: bool) {
        self.0 &= !(CCR_CIRC);
        if enable {
            self.0 |= CCR_CIRC;
        }
    }

    pub fn enable_peripheral_increment_mode(&mut self, enable: bool) {
        self.0 &= !(CCR_PINC);
        if enable {
            self.0 |= CCR_PINC;
        }
    }

    pub fn enable_memory_increment_mode(&mut self, enable: bool) {
        self.0 &= !(CCR_MINC);
        if enable {
            self.0 |= CCR_MINC;
        }
    }

    pub fn set_peripheral_size(&mut self, periph_size: PeriphAndMemSize) {
        let mask = match periph_size {
            PeriphAndMemSize::Eight => 0,
            PeriphAndMemSize::Sixteen => CCR_PSIZE0,
            PeriphAndMemSize::ThirtyTwo => CCR_PSIZE1,
            PeriphAndMemSize::Reserved => (CCR_PSIZE0 | CCR_PSIZE1),
        };

        self.0 &= !(CCR_PSIZE0 | CCR_PSIZE1);
        self.0 |= mask;
    }

    pub fn set_memory_size(&mut self, mem_size: PeriphAndMemSize) {
        let mask = match mem_size {
            PeriphAndMemSize::Eight => 0,
            PeriphAndMemSize::Sixteen => CCR_MSIZE0,
            PeriphAndMemSize::ThirtyTwo => CCR_MSIZE1,
            PeriphAndMemSize::Reserved => (CCR_MSIZE0 | CCR_MSIZE1),
        };

        self.0 &= !(CCR_MSIZE0 | CCR_MSIZE1);
        self.0 |= mask;
    }

    pub fn set_channel_priority(&mut self, chan_priority: ChannelPriorityLevel) {
        let mask = match chan_priority {
            ChannelPriorityLevel::Low => 0,
            ChannelPriorityLevel::Medium => CCR_PL0,
            ChannelPriorityLevel::High => CCR_PL1,
            ChannelPriorityLevel::VeryHigh => (CCR_PL0 | CCR_PL1),
        }

        self.0 &= !(CCR_PL0 | CCR_PL1);
        self.0 |= mask;
    }

    pub fn enable_mem2mem_mode(&mut self, enable: bool) {
        self.0 &= !(CCR_MEM2MEM);
        if enable {
            self.0 |= CCR_MEM2MEM;
        }
    }
}
