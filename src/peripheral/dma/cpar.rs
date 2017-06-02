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
    /// Set the peripheral address.
    ///
    /// This is the base address of the peripheral that is using the DMA.
    /// The data will be moved from/to this address to/from the memory after
    /// the peripheral event.
    pub fn set_pa(&mut self, periph_addr: *const u32) {
        self.0 = periph_addr as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpar_set_pa_correctly_stores_address() {
        let mut cpar = CPAR(0);
        assert_eq!(cpar.0, 0b0);

        cpar.set_pa(0x4001_3800);
        assert_eq!(cpar.0, 0x4001_3800);

        cpar.set_pa(0x4000_4400);
        assert_eq!(cpar.0, 0x4000_4400);
    }
}
