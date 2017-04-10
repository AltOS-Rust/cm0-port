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

//! This module handles the clock control register of the CRR.

use super::defs::*;

pub mod clock_rate {
    static mut CLOCK_RATE: u32 = 0;

    pub fn get_system_clock_rate() -> u32 {
        unsafe {
            CLOCK_RATE
        }
    }

    pub fn update_system_clock_rate() {
        use super::Clock;
        use super::super::super::systick;
        use super::super::defs::*;

        let rcc = super::super::rcc();
        let rate = match rcc.get_system_clock_source() {
            Clock::HSI => HSI_VALUE,
            Clock::HSE => HSE_VALUE,
            Clock::HSI48 => HSI48_VALUE,
            Clock::PLL => {
                let multiplier = rcc.get_pll_multiplier() as u32;
                let source = rcc.get_pll_source();
                let prediv_factor = rcc.get_pll_prediv_factor() as u32;

                match source {
                    Clock::HSE => (HSE_VALUE/prediv_factor) * multiplier,
                    Clock::HSI48 => (HSI48_VALUE/prediv_factor) * multiplier,
                    Clock::HSI => (HSI_VALUE/2) * multiplier,
                    _ => panic!("CRR::update_system_core_clock - invalid clock driving the PLL!"),
                }
            },
            _ => panic!("CRR::update_system_core_clock - invalid clock for the system clock!"),
        };

        unsafe { CLOCK_RATE = rate; }
        let mut systick = systick::systick();
        // Interrupt every millisecond
        systick.set_reload_value(rate / 1000);
    }
}

/// Defines available system clocks.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Clock {
    /// High Speed Internal: 8 MHz
    HSI,
    /// High Speed Internal: 48 MHz
    HSI48,
    /// High Speed Internal: 14 MHz
    HSI14,
    /// High Speed External: Variable Speed
    HSE,
    /// Phase Locked Loop: Variable Speed
    PLL,
}

/// The CR register only controls the PLL, HSE, and HSI clocks. If another clock is passed in as an
/// argument to any of the methods that take a clock argument, the kernel will panic.
#[derive(Copy, Clone, Debug)]
pub struct CR(u32);

impl CR {
    /// Set a clock to be on if `enable` is true, off otherwise.
    ///
    /// If `enable` is true, the return value is always true. If `enable` is false, the return
    /// value will be true if the clock was successfully disabled.
    ///
    /// # Panics
    ///
    /// If the clock passed in is not one of `PLL`, `HSE`, or `HSI`, this function will panic as
    /// the CR register only has control over those clocks.
    pub fn set_clock(&mut self, enable: bool, clock: Clock) -> bool {
        let mask = match clock {
            Clock::PLL => CR_PLLON,
            Clock::HSE => CR_HSEON,
            Clock::HSI => CR_HSION,
            _ => panic!("CR::enable_clock - argument clock is not controlled by this register!"),
        };

        if enable {
            self.0 |= mask;
            true
        }
        else {
            self.0 &= !mask;
            (self.0 & mask) == 0
        }
    }

    /// Return true if the specified clock is enabled.
    ///
    /// # Panics
    ///
    /// If the clock passed in is not one of `PLL`, `HSE`, or `HSI`, this function will panic as
    /// the CR register only has control over those clocks.
    pub fn clock_is_on(&self, clock: Clock) -> bool {
        let mask = match clock {
            Clock::PLL => CR_PLLON,
            Clock::HSE => CR_HSEON,
            Clock::HSI => CR_HSION,
            _ => panic!("CR::clock_is_on - argument clock is not controlled by thsi register!"),
        };

        (self.0 & mask) != 0
    }

    /// Return true if the specified clock is ready for use.
    ///
    /// # Panics
    ///
    /// If the clock passed in is not one of `PLL`, `HSE`, or `HSI`, this function will panic as
    /// the CR register only has control over those clocks.
    pub fn clock_is_ready(&self, clock: Clock) -> bool {
        let mask = match clock {
            Clock::PLL => CR_PLLRDY,
            Clock::HSE => CR_HSERDY,
            Clock::HSI => CR_HSIRDY,
            _ => panic!("CR::clock_is_ready - argument clock is not controlled by this register!"),
        };

        (self.0 & mask) != 0
    }
}

/// The CR2 register only controls the HSI48 and HSI14 clocks. If another clock is passed in as an
/// argument to any of the methods that take a clock argument, the kernel will panic.
#[derive(Copy, Clone, Debug)]
pub struct CR2(u32);

impl CR2 {
    /// Set a clock to be on if `enable` is true, off otherwise.
    ///
    /// If `enable` is true, the return value is always true. If `enable` is false, the return
    /// value will be true if the clock was successfully disabled.
    ///
    /// # Panics
    ///
    /// If the clock passed in is not one of `HSI14` or `HSI48`, this function will panic as
    /// the CR2 register only has control over those clocks.
    pub fn set_clock(&mut self, enable: bool, clock: Clock) -> bool {
        let mask = match clock {
            Clock::HSI48 => CR2_HSI48ON,
            Clock::HSI14 => CR2_HSI14ON,
            _ => panic!("CR2::set_clock - argument clock is not controlled by this register!"),
        };

        if enable {
            self.0 |= mask;
            true
        }
        else {
            self.0 &= !mask;
            (self.0 & mask) == 0
        }
    }

    /// Return true if the specified clock is enabled.
    ///
    /// # Panics
    ///
    /// If the clock passed in is not one of `HSI14` or `HSI48`, this function will panic as
    /// the CR2 register only has control over those clocks.
    pub fn clock_is_on(&self, clock: Clock) -> bool {
        let mask = match clock {
            Clock::HSI48 => CR2_HSI48ON,
            Clock::HSI14 => CR2_HSI14ON,
            _ => panic!("CR2::clock_is_on - argument clock is not controlled by this register!"),
        };

        (self.0 & mask) != 0
    }

    /// Return true if the specified clock is ready for use.
    ///
    /// # Panics
    ///
    /// If the clock passed in is not one of `HSI14` or `HSI48`, this function will panic as
    /// the CR2 register only has control over those clocks.
    pub fn clock_is_ready(&self, clock: Clock) -> bool {
        let mask = match clock {
            Clock::HSI48 => CR2_HSI48RDY,
            Clock::HSI14 => CR2_HSI14RDY,
            _ => panic!("CR2::clock_is_ready - argument clock is not controlled by this register!"),
        };

        (self.0 & mask) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cr_set_clock_pll_on() {
        let mut cr = CR(0);

        cr.set_clock(true, Clock::PLL);
        assert_eq!(cr.0, 0b1 << 24);
    }

    #[test]
    fn test_cr_set_clock_pll_off() {
        // PLL starts on
        let mut cr = CR(0b1 << 24);

        cr.set_clock(false, Clock::PLL);
        assert_eq!(cr.0, 0);
    }

    #[test]
    fn test_cr_set_clock_on_multiple_clocks_doesnt_change_other_clocks() {
        let mut cr = CR(0);

        cr.set_clock(true, Clock::PLL);
        assert_eq!(cr.0, 0b1 << 24);

        cr.set_clock(true, Clock::HSI);
        assert_eq!(cr.0, 0b1 | 0b1 << 24);
    }

    #[test]
    fn test_cr_set_clock_off_multiple_clocks_doesnt_change_other_clocks() {
        // HSI and PLL start on
        let mut cr = CR(0b1 | 0b1 << 24);

        cr.set_clock(false, Clock::HSI);
        assert_eq!(cr.0, 0b1 << 24);

        cr.set_clock(false, Clock::PLL);
        assert_eq!(cr.0, 0);
    }

    #[test]
    #[should_panic]
    fn test_cr_set_clock_unhandled_clock_panics() {
        let mut cr = CR(0);

        cr.set_clock(true, Clock::HSI48);
    }

    #[test]
    fn test_cr_clock_is_on_all_clocks_off() {
        let cr = CR(0);

        assert_eq!(cr.clock_is_on(Clock::PLL), false);
        assert_eq!(cr.clock_is_on(Clock::HSI), false);
        assert_eq!(cr.clock_is_on(Clock::HSE), false);
    }

    #[test]
    fn test_cr_clock_is_on_all_clocks_on() {
        // HSI, HSE, and PLL start on
        let cr = CR(0b1 | 0b1 << 16 | 0b1 << 24);

        assert_eq!(cr.clock_is_on(Clock::PLL), true);
        assert_eq!(cr.clock_is_on(Clock::HSI), true);
        assert_eq!(cr.clock_is_on(Clock::HSE), true);
    }

    #[test]
    #[should_panic]
    fn test_cr_clock_is_on_unhandled_clock_panics() {
        let cr = CR(0);

        cr.clock_is_on(Clock::HSI48);
    }

    #[test]
    fn test_cr_clock_is_ready_all_clocks_not_ready() {
        let cr = CR(0);

        assert_eq!(cr.clock_is_ready(Clock::PLL), false);
        assert_eq!(cr.clock_is_ready(Clock::HSI), false);
        assert_eq!(cr.clock_is_ready(Clock::HSE), false);
    }

    #[test]
    fn test_cr_clock_is_ready_all_clocks_ready() {
        // HSI, HSE, and PLL start ready
        let cr = CR(0b1 << 1 | 0b1 << 17 | 0b1 << 25);

        assert_eq!(cr.clock_is_ready(Clock::PLL), true);
        assert_eq!(cr.clock_is_ready(Clock::HSI), true);
        assert_eq!(cr.clock_is_ready(Clock::HSE), true);
    }

    #[test]
    #[should_panic]
    fn test_cr_clock_is_ready_unhandled_clock_panics() {
        let cr = CR(0);

        cr.clock_is_ready(Clock::HSI48);
    }

    #[test]
    fn test_cr2_set_hsi48_clock_on() {
        let mut cr2 = CR2(0);

        cr2.set_clock(true, Clock::HSI48);
        assert_eq!(cr2.0, 0b1 << 16);
    }

    #[test]
    fn test_cr2_set_hsi48_clock_off() {
        // HSI48 starts on
        let mut cr2 = CR2(0b1 << 16);

        cr2.set_clock(false, Clock::HSI48);
        assert_eq!(cr2.0, 0);
    }

    #[test]
    fn test_cr2_set_clock_on_multiple_clocks_doesnt_change_other_clocks() {
        let mut cr2 = CR2(0);

        cr2.set_clock(true, Clock::HSI48);
        assert_eq!(cr2.0, 0b1 << 16);

        cr2.set_clock(true, Clock::HSI14);
        assert_eq!(cr2.0, 0b1 | 0b1 << 16);
    }

    #[test]
    fn test_cr2_set_clock_off_multiple_clocks_doesnt_change_other_clocks() {
        // HSI14 and HSI48 start on
        let mut cr2 = CR2(0b1 | 0b1 << 16);

        cr2.set_clock(false, Clock::HSI14);
        assert_eq!(cr2.0, 0b1 << 16);

        cr2.set_clock(false, Clock::HSI48);
        assert_eq!(cr2.0, 0);
    }

    #[test]
    #[should_panic]
    fn test_cr2_set_clock_unhandled_clock_panics() {
        let mut cr2 = CR2(0);

        cr2.set_clock(true, Clock::PLL);
    }

    #[test]
    fn test_cr2_clock_is_on_all_clocks_off() {
        let cr2 = CR2(0);

        assert_eq!(cr2.clock_is_on(Clock::HSI48), false);
        assert_eq!(cr2.clock_is_on(Clock::HSI14), false);
    }

    #[test]
    fn test_cr2_clock_is_on_all_clocks_on() {
        // HSI14 and HSI48 start on
        let cr2 = CR2(0b1 | 0b1 << 16);

        assert_eq!(cr2.clock_is_on(Clock::HSI48), true);
        assert_eq!(cr2.clock_is_on(Clock::HSI14), true);
    }

    #[test]
    #[should_panic]
    fn test_cr2_clock_is_on_unhandled_clock_panics() {
        let cr2 = CR2(0);

        cr2.clock_is_on(Clock::PLL);
    }

    #[test]
    fn test_cr2_clock_is_ready_all_clocks_not_ready() {
        let cr2 = CR2(0);

        assert_eq!(cr2.clock_is_ready(Clock::HSI48), false);
        assert_eq!(cr2.clock_is_ready(Clock::HSI14), false);
    }

    #[test]
    fn test_cr2_clock_is_ready_all_clocks_ready() {
        // HSI14 and HSI48 start ready
        let cr2 = CR2(0b1 << 1 | 0b1 << 17);

        assert_eq!(cr2.clock_is_ready(Clock::HSI48), true);
        assert_eq!(cr2.clock_is_ready(Clock::HSI14), true);
    }

    #[test]
    #[should_panic]
    fn test_cr2_clock_is_ready_unhandled_clock_panics() {
        let cr2 = CR2(0);

        cr2.clock_is_ready(Clock::PLL);
    }
}
