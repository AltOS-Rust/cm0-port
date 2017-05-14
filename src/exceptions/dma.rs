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

use peripheral::dma::{DMA, DMAChannel, DMA_TX_CHAN4PLUS, DMA_RX_CHAN4PLUS};
use altos_core::syscall;

pub fn dma_tx(mut dma: DMA, chan: DMAChannel) {
    dma.channel_transfer_complete_clear(chan);
    dma[chan].disable_transmit_complete_interrupt();

    syscall::sys_wake(DMA_TX_CHAN4PLUS);
}
