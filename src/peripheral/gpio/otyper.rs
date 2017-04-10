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

/// Available GPIO pin types.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
    /// Actively drives the output to High.
    PushPull,
    /// Passively drives the output to High by an internal or external pull-up resistor.
    OpenDrain,
}

impl Field for Type {
    fn mask(&self) -> u32 {
        match *self {
            Type::PushPull => TYPE_PUSHPULL,
            Type::OpenDrain => TYPE_OPENDRAIN,
        }
    }
}

impl Type {
    fn from_mask(mask: u32) -> Self {
        match mask {
            TYPE_PUSHPULL => Type::PushPull,
            TYPE_OPENDRAIN => Type::OpenDrain,
            _ => panic!("Type::from_mask - mask was not a valid value!"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OTYPER(u32);

impl OTYPER {
    pub fn set_type(&mut self, new_type: Type, port: u8) {
        if port > 15 {
            panic!("OTYPER::set_type - specified port must be between [0..15]!");
        }

        match new_type {
            Type::PushPull => self.0 &= !(0b1 << port),
            Type::OpenDrain => self.0 |= 0b1 << port,
        }
    }

    pub fn get_type(&self, port: u8) -> Type {
        if port > 15 {
            panic!("OTYPER::get_type - specified port must be between [0..15]!");
        }

        let mask = (self.0 & (0b1 << port)) >> port;

        Type::from_mask(mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otyper_set_type() {
        let mut otyper = OTYPER(0);

        otyper.set_type(Type::OpenDrain, 4);
        assert_eq!(otyper.0, 0b1 << 4);
    }

    #[test]
    fn test_otyper_set_type_multiple_ports_doesnt_clear_settings() {
        let mut otyper = OTYPER(0);

        otyper.set_type(Type::OpenDrain, 5);
        assert_eq!(otyper.0, 0b1 << 5);

        otyper.set_type(Type::OpenDrain, 7);
        assert_eq!(otyper.0, 0b1 << 5 | 0b1 << 7);
    }

    #[test]
    #[should_panic]
    fn test_otyper_set_type_port_greater_than_15_panics() {
        let mut otyper = OTYPER(0);

        otyper.set_type(Type::OpenDrain, 16);
    }

    #[test]
    fn test_otyper_get_type() {
        let otyper = OTYPER(0b1 << 9);

        assert_eq!(otyper.get_type(9), Type::OpenDrain);
    }

    #[test]
    #[should_panic]
    fn test_otyper_get_type_port_greater_than_15_panics() {
        let otyper = OTYPER(0);

        otyper.get_type(16);
    }
}
