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
pub struct ISER(u32);
#[derive(Copy, Clone, Debug)]
pub struct ICER(u32);

impl ISER {
    pub fn enable_interrupt(&mut self, hardware: Hardware) {
        let interrupt = hardware as u8;

        self.0 |= 0b1 << interrupt;
    }

    pub fn interrupt_is_enabled(&self, hardware: Hardware) -> bool {
        let interrupt = hardware as u8;
        self.0 & (0b1 << interrupt) != 0
    }
}

impl ICER {
    pub fn disable_interrupt(&mut self, hardware: Hardware) {
        let interrupt = hardware as u8;

        self.0 |= 0b1 << interrupt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iser_enable_interrupt() {
        let mut iser = ISER(0);

        iser.enable_interrupt(Hardware::Flash);
        assert_eq!(iser.0, 0b1 << 3);
    }

    #[test]
    fn test_iser_interrupt_is_enabled() {
        let iser = ISER(0b1 << 5);

        assert!(iser.interrupt_is_enabled(Hardware::Exti01));
        assert!(!iser.interrupt_is_enabled(Hardware::Usb));
    }

    #[test]
    fn test_icer_disable_interrupt() {
        let mut icer = ICER(0);

        icer.disable_interrupt(Hardware::Flash);
        assert_eq!(icer.0, 0b1 << 3);
    }
}
