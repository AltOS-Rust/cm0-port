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

use super::super::Register;
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
