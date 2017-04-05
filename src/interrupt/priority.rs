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

use peripheral::Field;
use super::defs::*;

/// The priority of the interrupt.
///
/// If in the interrupt handler and another interrupt with a higher priority is generated,
/// the CPU will handle the higher priority interrupt before it finishes handling the
/// lower priority interrupt.
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Priority {
    Highest,
    High,
    Low,
    Lowest,
}

impl Field for Priority {
    fn mask(&self) -> u32 {
        match *self {
            Priority::Highest => IPR_PRIORITY_HIGHEST,
            Priority::High => IPR_PRIORITY_HIGH,
            Priority::Low => IPR_PRIORITY_LOW,
            Priority::Lowest => IPR_PRIORITY_LOWEST,
        }
    }
}

impl Priority {
    fn from_mask(mask: u32) -> Self {
        match mask {
            IPR_PRIORITY_HIGHEST => Priority::Highest,
            IPR_PRIORITY_HIGH => Priority::High,
            IPR_PRIORITY_LOW => Priority::Low,
            IPR_PRIORITY_LOWEST => Priority::Lowest,
            _ => panic!("Priority::from_mask - mask was not a valid value!"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IPR(u32);

impl IPR {
    pub fn set_priority(&mut self, priority: Priority, interrupt: u8) {
        assert!(interrupt < 4);
        let mask = priority.mask();
        let interrupt_shift = interrupt * 8;

        self.0 &= !(IPR_PRIORITY_FIELD_MASK << interrupt_shift);
        self.0 |= mask << interrupt_shift;
    }

    pub fn get_priority(&self, interrupt:u8) -> Priority {
        assert!(interrupt < 4);
        let interrupt_shift = interrupt * 8;
        let mask = (self.0 & IPR_PRIORITY_FIELD_MASK << interrupt_shift) >> interrupt_shift;
        Priority::from_mask(mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipr_set_priority() {
        let mut ipr = IPR(0);

        ipr.set_priority(Priority::High, 1);
        assert_eq!(ipr.0, 0b01 << 14);
    }

    #[test]
    #[should_panic]
    fn test_ipr_set_priority_offset_greater_than_4_panics() {
        let mut ipr = IPR(0);

        ipr.set_priority(Priority::High, 5);
    }

    fn test_ipr_get_priority() {
        let ipr = IPR(0b11 << 6);

        assert_eq!(ipr.get_priority(0), Priority::Lowest);
    }
}
