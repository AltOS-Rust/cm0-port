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
    /// Set a clock to be on if `enable` is true, off otherwise. If `enable` is true, the return
    /// value is always true. If `enable` is false, the return value will be true if the clock was
    /// successfully disabled.
    pub fn set_clock(&mut self, enable: bool, clock: Clock) -> bool {
        let mask = match clock {
            Clock::PLL => PLLON,
            Clock::HSE => HSEON,
            Clock::HSI => HSION,
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
    pub fn clock_is_on(&self, clock: Clock) -> bool {
        let mask = match clock {
            Clock::PLL => PLLON,
            Clock::HSE => HSEON,
            Clock::HSI => HSION,
            _ => panic!("CR::clock_is_on - argument clock is not controlled by thsi register!"),
        };

        (self.0 & mask) != 0
    }

    /// Return true if the specified clock is ready for use.
    pub fn clock_is_ready(&self, clock: Clock) -> bool {
        let mask = match clock {
            Clock::PLL => PLLRDY,
            Clock::HSE => HSERDY,
            Clock::HSI => HSIRDY,
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
    /// Set a clock to be on if `enable` is true, off otherwise. If `enable` is true, the return
    /// value is always true. If `enable` is false, the return value will be true if the clock was
    /// successfully disabled.
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
    pub fn clock_is_on(&self, clock: Clock) -> bool {
        let mask = match clock {
            Clock::HSI48 => CR2_HSI48ON,
            Clock::HSI14 => CR2_HSI14ON,
            _ => panic!("CR2::clock_is_on - argument clock is not controlled by this register!"),
        };

        (self.0 & mask) != 0
    }

    /// Return true if the specified clock is ready for use.
    pub fn clock_is_ready(&self, clock: Clock) -> bool {
        let mask = match clock {
            Clock::HSI48 => CR2_HSI48RDY,
            Clock::HSI14 => CR2_HSI14RDY,
            _ => panic!("CR2::clock_is_ready - argument clock is not controlled by this register!"),
        };

        (self.0 & mask) != 0
    }
}
