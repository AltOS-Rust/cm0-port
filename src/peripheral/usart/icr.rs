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

/* This submodule contains the function implementations for the ICR.
 * The ICR is the interrupt clear register and is responsible for
 * clearing various interrupt flags that are generated in the ISR.
 * It does so by writing a 1 to specific bits in this register.
 */

use super::defs::*;

#[derive(Debug)]
pub struct ICR(u32);

impl ICR {
    /*  Bit 3 ORECF: Overrun error clear flag
     *  Writing 1 to this bit clears the ORE flag in the USARTx_ISR.
     */
    pub fn clear_ore(&mut self) {
        self.0 |= ICR_ORECF;
    }

    /* Bit 4 IDLECF: Idle line detected clear flag
     * Writing 1 to this bit clears the IDLE flag in the USARTx_ISR.
     */
    pub fn clear_idle(&mut self) {
        self.0 |= ICR_IDLECF;
    }

    /* Bit 6 TCCF: Transmission complete clear flag
     * Writing 1 to this bit clears the TC flag in the USARTx_ISR.
     */
    pub fn clear_tc(&mut self) {
        self.0 |= ICR_TCCF;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icr_clear_ore() {
        let mut icr = ICR(0);
        icr.clear_ore();

        assert_eq!(icr.0, 0b1 << 3);
    }

    #[test]
    fn test_icr_clear_tc() {
        let mut icr = ICR(0);
        icr.clear_tc();

        assert_eq!(icr.0, 0b1 << 6);
    }
}
