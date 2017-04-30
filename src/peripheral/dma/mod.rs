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

use core::ops::{Deref, DerefMut};
use volatile::Volatile;
use self::ccr::CCR;
use self::cndtr::CNDTR;
use self::cpar::CPAR;
use self::cmar::CMAR;
use self::ifcr::IFCR;
use self::defs::*;

#[derive(Copy, Clone, Debug)]
struct DMAChannel {
    ccr: CCR,
    cndtr: CNDTR,
    cpar: CPAR,
    cmar: CMAR,
    _res: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[doc(hidden)]
pub struct RawDMA {
    isr: u32,
    ifcr: IFCR,
    channel: [DMAChannel; 5]
}

#[derive(Copy, Clone, Debug)]
pub struct DMA(Volatile<RawDMA>);

impl DMA {

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
