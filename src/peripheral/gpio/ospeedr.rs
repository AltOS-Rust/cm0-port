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

/// Defines available GPIO speeds.
///
/// Refer to the device data sheet for the frequency specifications
/// and the power supply and load conditions for each speed.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Speed {
    Low,
    Medium,
    High,
}

impl Field for Speed {
    fn mask(&self) -> u32 {
        match *self {
            Speed::Low => SPEED_LOW,
            Speed::Medium => SPEED_MEDIUM,
            Speed::High => SPEED_HIGH,
        }
    }
}

impl Speed {
    fn from_mask(mask: u32) -> Self {
        match mask {
            SPEED_LOW | SPEED_LOW_ALT => Speed::Low,
            SPEED_MEDIUM => Speed::Medium,
            SPEED_HIGH => Speed::High,
            _ => panic!("Speed::from_mask - mask was not a valid value!"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OSPEEDR(u32);

impl OSPEEDR {
    pub fn set_speed(&mut self, speed: Speed, port: u8) {
        if port > 15 {
            panic!("OSPEEDR::set_speed - specified port must be between [0..15]!");
        }
        let mask = speed.mask();

        self.0 &= !(SPEED_MASK << (port * 2));
        self.0 |= mask << (port * 2);
    }

    pub fn get_speed(&self, port: u8) -> Speed {
        if port > 15 {
            panic!("OSPEEDR::get_speed - specified port must be between [0..15]!");
        }

        let mask = (self.0 & (SPEED_MASK << (port * 2))) >> (port * 2);

        Speed::from_mask(mask)
    }
}
