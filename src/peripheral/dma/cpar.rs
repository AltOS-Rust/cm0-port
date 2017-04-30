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

#[derive(Copy, Clone, Debug)]
pub struct CPAR(u32);

impl CPAR {
    /* Bits 31:0 PA[31:0]: Peripheral address
     * Base address of the peripheral data register from/to which the data will be
     *   read/written.
     * When PSIZE is 01 (16-bit), the PA[0] bit is ignored.
     *   Access is automatically aligned to a half-word address.
     * When PSIZE is 10 (32-bit), PA[1:0] are ignored.
     *   Access is automatically aligned to a word address.
    */
    pub fn set_pa(&mut self, periph_addr: u32) {
        self.0 = periph_addr;
    }
}

