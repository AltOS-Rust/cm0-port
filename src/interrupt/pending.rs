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

use interrupt::Hardware;

#[derive(Copy, Clone, Debug)]
pub struct ISPR(u32);
#[derive(Copy, Clone, Debug)]
pub struct ICPR(u32);

impl ISPR {
    pub fn set_pending(&mut self, hardware: Hardware) {
        let interrupt = hardware as u8;

        self.0 |= 0b1 << interrupt;
    }

    pub fn interrupt_is_pending(&self, hardware: Hardware) -> bool {
        let interrupt = hardware as u8;

        self.0 & (0b1 << interrupt) != 0
    }
}

impl ICPR {
    pub fn clear_pending(&mut self, hardware: Hardware) {
        let interrupt = hardware as u8;

        self.0 |= 0b1 << interrupt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ispr_set_pending() {
        let mut ispr = ISPR(0);

        ispr.set_pending(Hardware::Flash);
        assert_eq!(ispr.0, 0b1 << 3);
    }

    #[test]
    fn test_ispr_interrupt_is_pending() {
        let ispr = ISPR(0b1 << 5);

        assert!(ispr.interrupt_is_pending(Hardware::Exti01));
        assert!(!ispr.interrupt_is_pending(Hardware::Usb));
    }

    #[test]
    fn test_icpr_clear_pending() {
        let mut icpr = ICPR(0);

        icpr.clear_pending(Hardware::Flash);
        assert_eq!(icpr.0, 0b1 << 3);
    }
}
