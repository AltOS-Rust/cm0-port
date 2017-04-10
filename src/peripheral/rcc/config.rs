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

//! This module handles the CFGR register, which deals with clock configuration.

use super::Clock;
use super::defs::*;

/// Clock Configuration Register
#[derive(Copy, Clone, Debug)]
pub struct CFGR(u32);

impl CFGR {
    pub fn get_system_clock_source(&self) -> Clock {
        let set_bits = self.0 & CFGR_SWS_MASK;

        match set_bits {
            CFGR_SWS_HSI => Clock::HSI,
            CFGR_SWS_HSE => Clock::HSE,
            CFGR_SWS_PLL => Clock::PLL,
            CFGR_SWS_HSI48 => Clock::HSI48,
            _    => panic!("CFGR::get_system_clock_source - set bits gave an unknown value!"),
        }
    }

    pub fn set_system_clock_source(&mut self, clock: Clock) {
        let mask = match clock {
            Clock::HSI => CFGR_CLOCK_HSI,
            Clock::HSE => CFGR_CLOCK_HSE,
            Clock::PLL => CFGR_CLOCK_PLL,
            Clock::HSI48 => CFGR_CLOCK_HSI48,
            _ => panic!("CFGR::set_system_clock_source - the clock argument cannot be used as a source!"),
        };

        // Zero the selection first (does this have any side effects)?
        self.0 &= !CFGR_SW_CLEAR_MASK;
        self.0 |= mask;
    }

    pub fn get_pll_source(&self) -> Clock {
        let set_bits = self.0 & CFGR_PLLSRC_MASK;

        match set_bits {
            CFGR_PLLSRC_HSI_2 | CFGR_PLLSRC_HSI_PREDIV => Clock::HSI,
            CFGR_PLLSRC_HSE_PREDIV => Clock::HSE,
            CFGR_PLLSRC_HSI48_PREDIV => Clock::HSI48,
            _ => panic!("CFGR::get_pll_source - set bits gave an unknown value!"),
        }
    }

    pub fn set_pll_source(&mut self, clock: Clock) {
        let mask = match clock {
            Clock::HSI   => CFGR_PLLSRC_HSI_2,
            Clock::HSE   => CFGR_PLLSRC_HSE_PREDIV,
            Clock::HSI48 => CFGR_PLLSRC_HSI48_PREDIV,
            _ => panic!("CFGR::set_pll_source - the clock argument cannot be used as a source!"),
        };

        // Zero the register first
        self.0 &= !CFGR_PLLSRC_MASK;
        self.0 |= mask;
    }

    pub fn get_pll_multiplier(&self) -> u8 {
        let set_bits = (self.0 & CFGR_PLLMUL_MASK) >> 18;

        // Just the way the multiplier is set up...
        let mut mul = set_bits + 2;
        if mul > 16 {
            mul = 16
        }
        mul as u8
    }

    pub fn set_pll_multiplier(&mut self, mul: u8) {
        if mul < 2 || mul > 16 {
            panic!("CFGR::set_pll_multiplier - the multiplier must be between 2..16!");
        }
        let mask = ((mul - 2) as u32) << 18;

        // Zero the register field
        self.0 &= !CFGR_PLLMUL_MASK;
        self.0 |= mask;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CFGR2(u32);

impl CFGR2 {
    pub fn get_pll_prediv_factor(&self) -> u8 {
        let set_bits = self.0 & CFGR2_PREDIV_MASK;

        // Division factor is 1 greater than the value of the bits set
        (set_bits + 1) as u8
    }

    pub fn set_pll_prediv_factor(&mut self, factor: u8) {
        if factor < 1 || factor > 16 {
            panic!("CFGR2::set_pll_prediv_factor - the division factor must be between 1..16!");
        }
        let mask = (factor - 1) as u32;

        // Zero the register field
        self.0 &= !CFGR2_PREDIV_MASK;
        self.0 |= mask;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfgr_get_system_clock_source() {
        // PLL starts as source
        let cfgr = CFGR(0b10 << 2);

        assert_eq!(cfgr.get_system_clock_source(), Clock::PLL);
    }

    #[test]
    fn test_cfgr_set_system_clock_source() {
        let mut cfgr = CFGR(0);

        cfgr.set_system_clock_source(Clock::PLL);
        assert_eq!(cfgr.0, 0b10);
    }

    #[test]
    #[should_panic]
    fn test_cfgr_set_system_clock_source_invalid_clock_panics() {
        let mut cfgr = CFGR(0);

        cfgr.set_system_clock_source(Clock::HSI14);
    }

    #[test]
    fn test_cfgr_get_pll_source() {
        let cfgr = CFGR(0b01 << 15);

        assert_eq!(cfgr.get_pll_source(), Clock::HSI);
    }

    #[test]
    fn test_cfgr_set_pll_source() {
        let mut cfgr = CFGR(0);

        cfgr.set_pll_source(Clock::HSE);
        assert_eq!(cfgr.0, 0b10 << 15);
    }

    #[test]
    #[should_panic]
    fn test_cfgr_set_pll_source_invalid_clock_panics() {
        let mut cfgr = CFGR(0);

        cfgr.set_pll_source(Clock::PLL);
    }

    #[test]
    fn test_cfgr_get_pll_multiplier() {
        // Multiplier field starts at 8, should get 10 out
        let cfgr = CFGR(0b1000 << 18);

        assert_eq!(cfgr.get_pll_multiplier(), 10);
    }

    #[test]
    fn test_cfgr_get_pll_multiplier_saturates_at_16() {
        // Multiplier field starts at 15, should get 16 out
        let cfgr = CFGR(0b1111 << 18);

        assert_eq!(cfgr.get_pll_multiplier(), 16);
    }

    #[test]
    fn test_cfgr_set_pll_multiplier() {
        let mut cfgr = CFGR(0);

        cfgr.set_pll_multiplier(8);
        assert_eq!(cfgr.0, 0b0110 << 18);
    }

    #[test]
    #[should_panic]
    fn test_cfgr_set_pll_multiplier_less_than_2_panics() {
        let mut cfgr = CFGR(0);

        cfgr.set_pll_multiplier(1);
    }

    #[test]
    #[should_panic]
    fn test_cfgr_set_pll_multiplier_greater_than_16_panics() {
        let mut cfgr = CFGR(0);

        cfgr.set_pll_multiplier(17);
    }

    #[test]
    fn test_cfgr2_get_pll_prediv_factor() {
        // Prediv factor field starts at 12, should get 13 out
        let cfgr2 = CFGR2(0b1100);

        assert_eq!(cfgr2.get_pll_prediv_factor(), 13);
    }

    #[test]
    fn test_cfgr2_set_pll_prediv_factor() {
        let mut cfgr2 = CFGR2(0);

        cfgr2.set_pll_prediv_factor(10);
        assert_eq!(cfgr2.0, 0b1001);
    }

    #[test]
    #[should_panic]
    fn test_cfgr2_set_pll_prediv_factor_0_panics() {
        let mut cfgr2 = CFGR2(0);

        cfgr2.set_pll_prediv_factor(0);
    }

    #[test]
    #[should_panic]
    fn test_cfgr2_set_pll_prediv_factor_greater_than_16_panics() {
        let mut cfgr2 = CFGR2(0);

        cfgr2.set_pll_prediv_factor(17);
    }
}
