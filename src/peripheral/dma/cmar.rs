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
pub struct CMAR(u32);

impl CMAR {
    /* Bits 31:0 MA[31:0]: Memory address Base address of the memory area from/to which
     *   the data will be read/written.
     * When MSIZE is 01 (16-bit), the MA[0] bit is ignored. Access is automatically
     *   aligned to a half-word address.
     * When MSIZE is 10 (32-bit), MA[1:0] are ignored. Access is automatically aligned
     *   to a word address.
     */
    pub fn set_ma(&mut self, mem_addr: u32) {
        self.0 = mem_addr;
    }
}
