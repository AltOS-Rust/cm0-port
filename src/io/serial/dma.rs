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


use altos_core::sync::CriticalSection;
use core::fmt::{self, Write};
use peripheral::dma::{self, DMAChannel, DMA_TX_CHAN4PLUS};
use peripheral::usart::{Usart, UsartX};
use super::WRITE_LOCK;

struct DMASerial {
    usart: Usart,
}

impl DMASerial {
    fn new(usart: Usart) -> Self {
        DMASerial { usart: usart }
    }
}

impl Write for DMASerial {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        use peripheral::usart::defs::*;

        let g = CriticalSection::begin();
        dma::set_dma_usart_tx(DMAChannel::Four,
                          unsafe {USART2_ADDR.offset((TDR_OFFSET/4) as isize)},
                          string.as_bytes());

        ::altos_core::syscall::sys_sleep(DMA_TX_CHAN4PLUS);
        drop(g);
        Ok(())
    }
}

#[doc(hidden)]
pub fn dma_write_str(s: &str) {
    let usart2 = Usart::new(UsartX::Usart2);
    let mut dma_serial = DMASerial::new(usart2);

    let _g = WRITE_LOCK.lock();
    dma_serial.write_str(s).ok();
}
