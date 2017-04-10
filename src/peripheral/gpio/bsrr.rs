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
pub struct BSRR(u32);

impl BSRR {
    /// Set the bit high for the specified port, port must be a value between [0..15] or the kernel
    /// will panic.
    pub fn set(&mut self, port: u8) {
        if port > 15 {
            panic!("BSRR::set - specified port must be between [0..15]!");
        }
        self.0 |= 0b1 << port;
    }

    pub fn reset(&mut self, port: u8) {
        if port > 15 {
            panic!("BSRR::reset - specified port must be between [0..15]!");
        }
        self.0 |= 0b1 << (port + BSRR_RESET_OFFSET);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsrr_set_8_yields_binary_256() {
        let mut bsrr = BSRR(0);
        bsrr.set(8);
        assert_eq!(bsrr.0, 0x100);
    }

    #[test]
    fn test_bsrr_set_3_yields_binary_8() {
        let mut bsrr = BSRR(0);
        bsrr.set(3);
        assert_eq!(bsrr.0, 0x08);
    }

    #[test]
    #[should_panic]
    fn test_bsrr_set_panics_when_port_is_out_of_bounds() {
        let mut bsrr = BSRR(0);
        bsrr.set(16);
    }

    #[test]
    fn test_bsrr_reset_port_0_yields_correct_value() {
        let mut bsrr = BSRR(0);
        bsrr.reset(0);
        assert_eq!(bsrr.0, 0x10000);
    }

    #[test]
    fn test_bsrr_reset_port_15_yields_correct_value() {
        let mut bsrr = BSRR(0);
        bsrr.reset(15);
        assert_eq!(bsrr.0, 0x8000_0000);
    }

    #[test]
    #[should_panic]
    fn test_bsrr_reset_panics_when_port_is_out_of_bounds() {
        let mut bsrr = BSRR(0);
        bsrr.set(16);
    }
}
