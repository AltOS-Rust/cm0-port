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

use super::super::{Register, Field};
use super::defs::*;

/// Defines available modes for the GPIO pins.
#[derive(Copy, Clone)]
pub enum Mode {
    /// Pin configured for input data.
    Input,
    /// Pin configured for output data.
    Output,
    /// Pin configured for an alternate function.
    Alternate,
    /// Pin configured to receive analog signal.
    Analog,
}

impl Field for Mode {
    fn mask(&self) -> u32 {
        match *self {
            Mode::Input => MODE_INPUT,
            Mode::Output => MODE_OUTPUT,
            Mode::Alternate => MODE_ALTERNATE,
            Mode::Analog => MODE_ANALOG,
        }
    }
}

impl Mode {
    fn from_mask(mask: u32) -> Self {
        match mask {
            MODE_INPUT => Mode::Input,
            MODE_OUTPUT => Mode::Output,
            MODE_ALTERNATE => Mode::Alternate,
            MODE_ANALOG => Mode::Analog,
            _ => panic!("Mode::from_mask - mask was not a valid value!"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MODER(u32);

impl MODER {
    pub fn set_mode(&mut self, mode: Mode, port: u8) {
        if port > 15 {
            panic!("MODER::set_mode - specified port must be a value between [0..15]!");
        }
        let mask = mode.mask();

        self.0 &= !(MODE_MASK << (port * 2));
        self.0 |= mask << (port * 2);
    }

    /// Get the current mode for the specified port, port must be a value between [0..15] or
    /// the kernel will panic.
    pub fn get_mode(&self, port: u8) -> Mode {
        if port > 15 {
            panic!("MODER::get_mode - specified port must be a value between [0..15]!");
        }

        let mask = (self.0 & (MODE_MASK << (port * 2))) >> (port * 2);

        Mode::from_mask(mask)
    }
}
