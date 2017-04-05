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

/// Set the functionality of a port.
///
/// See data sheet for port mappings.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum AlternateFunction {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl Field for AlternateFunction {
    fn mask(&self) -> u32 {
        match *self {
            AlternateFunction::Zero => AF0,
            AlternateFunction::One => AF1,
            AlternateFunction::Two => AF2,
            AlternateFunction::Three => AF3,
            AlternateFunction::Four => AF4,
            AlternateFunction::Five => AF5,
            AlternateFunction::Six => AF6,
            AlternateFunction::Seven => AF7,
        }
    }
}

impl AlternateFunction {
    fn from_mask(mask: u32) -> Self {
        match mask {
            AF0 => AlternateFunction::Zero,
            AF1 => AlternateFunction::One,
            AF2 => AlternateFunction::Two,
            AF3 => AlternateFunction::Three,
            AF4 => AlternateFunction::Four,
            AF5 => AlternateFunction::Five,
            AF6 => AlternateFunction::Six,
            AF7 => AlternateFunction::Seven,
            _ => panic!("AlternateFunction::from_mask - mask was not a valid value!"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AFRL(u32);
impl AFRL {
    pub fn set_function(&mut self, function: AlternateFunction, port: u8) {
        let mask = function.mask();

        self.0 &= !(AFR_MASK << (port * 4));
        self.0 |= mask << (port * 4);
    }

    pub fn get_function(&self, port: u8) -> AlternateFunction {
        let mask = self.0 & (AFR_MASK << (port * 4));

        AlternateFunction::from_mask(mask)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AFRH(u32);
impl AFRH {
    pub fn set_function(&mut self, function: AlternateFunction, port: u8) {
        let mask = function.mask();

        // #9: Port needs to be subtracted by 8 since afr registers are split into high and low
        // for 0-7 and 8-15. i.e. port 9 is actually offset 1 * 4 in the afrh register
        // (rather than offset 9 * 4)
        let port = port - 8;
        self.0 &= !(AFR_MASK << (port * 4));
        self.0 |= mask << (port * 4);
    }

    pub fn get_function(&self, port: u8) -> AlternateFunction {
        // #9: See comment in `set_function`
        let port = port - 8;
        let mask = self.0 & (AFR_MASK << (port * 4));

        AlternateFunction::from_mask(mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_afrh_set_function() {
        let mut afrh = AFRH(0);
        afrh.set_function(AlternateFunction::Five, 8);

        assert_eq!(afrh.0, 0x5);
    }

    #[test]
    #[should_panic]
    fn test_afrh_set_port_out_of_bounds_panics() {
        let mut afrh = AFRH(0);
        afrh.set_function(AlternateFunction::Seven, 2);
    }

    #[test]
    fn test_afrl_set_function() {
        let mut afrl = AFRL(0);
        afrl.set_function(AlternateFunction::Two, 3);

        assert_eq!(afrl.0, 0x2000);
    }

    #[test]
    #[should_panic]
    fn test_afrl_set_port_out_of_bounds_panics() {
        let mut afrl = AFRL(0);
        afrl.set_function(AlternateFunction::Two, 10);
    }
}
