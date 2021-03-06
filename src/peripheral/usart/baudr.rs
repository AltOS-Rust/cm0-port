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

/* This submodule contains the function implementations for the Usartx_BRR.
 * The BRR is the baud rate register and is responsible for setting the
 * baud rate based on what the user needs.
 */

use super::defs::*;

/// Five most common baud rates available.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum BaudRate {
    Hz4800,
    Hz9600,
    Hz19200,
    Hz57600,
    Hz115200,
}

#[derive(Copy, Clone, Debug)]
pub struct BRR(u32);

impl BRR {
    /* Bits 31:16 Reserved, must be kept at reset value.
     * Bits 15:4 BRR[15:4]
     *   BRR[15:4] = USARTDIV[15:4]
     * Bits 3:0 BRR[3:0]
     *   When OVER8 = 0, BRR[3:0] = USARTDIV[3:0].
     *   When OVER8 = 1:
     *   BRR[2:0] = USARTDIV[3:0] shifted 1 bit to the right.
     *   BRR[3] must be kept cleared.
     */
    pub fn set_baud_rate(&mut self, baud_rate: BaudRate, clock_rate: u32, over8: bool) {
        let mut rate = match baud_rate {
            BaudRate::Hz4800 => clock_rate/4_800,
            BaudRate::Hz9600 => clock_rate/9_600,
            BaudRate::Hz19200 => clock_rate/19_200,
            BaudRate::Hz57600 => clock_rate/57_600,
            BaudRate::Hz115200 => clock_rate/115_200,
        };

        if over8 {
            let mut low_bits = rate & DIV_MASK;
            low_bits = low_bits >> 1;
            rate &= !(DIV_MASK);
            rate |= low_bits;
        }

        self.0 = rate;
    }
}
