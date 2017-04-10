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

/// The clock source for the SysTick device
pub enum ClockSource {
    /// Use a reference clock
    Reference,
    /// Use the main system clock
    Processor,
}

/// The control and status register for the SysTick timer.
#[derive(Copy, Clone, Debug)]
pub struct CSR(u32);

impl CSR {
    pub fn set_enable(&mut self, enable: bool) {
        if enable {
            self.0 |= ENABLE;
        }
        else {
            self.0 &= !ENABLE;
        }
    }

    pub fn set_interrupt(&mut self, enable: bool) {
        if enable {
            self.0 |= TICKINT;
        }
        else {
            self.0 &= !TICKINT;
        }
    }

    pub fn set_source(&mut self, source: ClockSource) {
        match source {
            ClockSource::Reference => self.0 &= !CLKSOURCE,
            ClockSource::Processor => self.0 |= CLKSOURCE,
        };
    }

    /// Returns true if the counter has reached zero since the last time it was checked.
    pub fn did_underflow(&self) -> bool {
        (self.0 & COUNTFLAG) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csr_set_enable_on() {
        let mut csr = CSR(0);

        csr.set_enable(true);
        assert_eq!(csr.0, 0b1);
    }

    #[test]
    fn test_csr_set_enable_off() {
        let mut csr = CSR(0b1);

        csr.set_enable(false);
        assert_eq!(csr.0, 0);
    }

    #[test]
    fn test_csr_set_interrupt_on() {
        let mut csr = CSR(0);

        csr.set_interrupt(true);
        assert_eq!(csr.0, 0b1 << 1);
    }

    #[test]
    fn test_csr_set_interrupt_off() {
        let mut csr = CSR(0);

        csr.set_interrupt(false);
        assert_eq!(csr.0, 0);
    }

    #[test]
    fn test_csr_set_source() {
        let mut csr = CSR(0);

        csr.set_source(ClockSource::Processor);
        assert_eq!(csr.0, 0b1 << 2);
    }

    #[test]
    fn test_csr_did_underflow_false_if_underflow_bit_not_set() {
        let csr = CSR(0);

        assert_eq!(csr.did_underflow(), false);
    }

    #[test]
    fn test_csr_did_underflow_true_if_underflow_bit_set() {
        // underflow bit set at start
        let csr = CSR(0b1 << 16);

        assert_eq!(csr.did_underflow(), true);
    }
}
