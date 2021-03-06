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

use super::super::Field;
use super::defs::*;

/// Defines the behavior of the GPIO pin when not asserted.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pull {
    /// No behavior.
    Neither,
    /// Pull toward high voltage.
    Up,
    /// Pull toward low voltage.
    Down,
}

impl Field for Pull {
    fn mask(&self) -> u32 {
        match *self {
            Pull::Neither => PUPD_NEITHER,
            Pull::Up => PUPD_UP,
            Pull::Down => PUPD_DOWN,
        }
    }
}

impl Pull {
    fn from_mask(mask: u32) -> Self {
        match mask {
            PUPD_NEITHER => Pull::Neither,
            PUPD_UP => Pull::Up,
            PUPD_DOWN => Pull::Down,
            _ => panic!("Pull::from_mask - mask was an invalid value!"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PUPDR(u32);

impl PUPDR {
    pub fn set_pull(&mut self, pull: Pull, port: u8) {
        if port > 15 {
            panic!("PUPDR::set_pull - specified port must be between [0..15]!");
        }
        let mask = pull.mask();

        self.0 &= !(PUPD_MASK << (port * 2));
        self.0 |= mask << (port * 2);
    }

    pub fn get_pull(&self, port: u8) -> Pull {
        if port > 15 {
            panic!("PUPDR::get_pull - specified port must be between [0..15]!");
        }

        let mask = (self.0 & (PUPD_MASK << (port * 2))) >> (port * 2);

        Pull::from_mask(mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pupdr_set_pull() {
        let mut pupdr = PUPDR(0);

        pupdr.set_pull(Pull::Up, 4);
        assert_eq!(pupdr.0, 0b01 << 8);
    }

    #[test]
    fn test_pupdr_set_pull_multiple_ports_doesnt_clear_settings() {
        let mut pupdr = PUPDR(0);

        pupdr.set_pull(Pull::Down, 5);
        assert_eq!(pupdr.0, 0b10 << 10);

        pupdr.set_pull(Pull::Up, 6);
        assert_eq!(pupdr.0, 0b10 << 10 | 0b01 << 12);
    }

    #[test]
    #[should_panic]
    fn test_pupdr_set_pull_port_greater_than_15_panics() {
        let mut pupdr = PUPDR(0);

        pupdr.set_pull(Pull::Up, 16);
    }

    #[test]
    fn test_pupdr_get_pull() {
        let pupdr = PUPDR(0b10 << 6);

        assert_eq!(pupdr.get_pull(3), Pull::Down);
    }

    #[test]
    #[should_panic]
    fn test_pupdr_get_pull_port_greater_than_15_panics() {
        let pupdr = PUPDR(0);

        pupdr.get_pull(16);
    }
}
