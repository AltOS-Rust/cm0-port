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

use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct CNTR {
    base_addr: *const u32,
}

/*
 * CNTR is the register that stores the values for the general behavior of the USB peripheral
 * Functionality such as; turning off device, enabling interrupts, low power mode,etc
 * We use bit wise operations on the register to flip values to determine what function is
 * enabled/disabled
 */

pub struct CNTR(u32);

impl CNTR {
    // NOTE: Not sure what this is for.
    pub fn set(&self,port:u32) {
        unsafe {
            let mut reg = self.addr();
            reg.store(port);
        }
    }

    // NOTE: Not sure what this is for.
    pub fn get(&self) -> *const u32 {
        unsafe {
            self.addr().as_ptr()
        }
    }

    // NOTE: Should this live here??
    pub fn enable_interrupts(&self) {
        unsafe {
            let mut reg = self.addr();
            // NOTE: This isn't right... all these things don't need to be set.
            //       Default reset for control register = 0x03
            // reg.store(CNTR_CTRM | !CNTR_PMAOVRM | !CNTR_WKUPM |
            //           !CNTR_SUSPM | CNTR_RESETM | !CNTR_SOFM |
            //           !CNTR_ESOFM | !CNTR_RESUME | !CNTR_FSUSP |
            //           !CNTR_LP_MODE | !CNTR_PDWN | !CNTR_FRES
        }
    }

    // Force reset the USB peripheral.
    pub fn force_reset(&mut self) {
        self.reset_control(true);
    }

    // Clear the USB reset.
    pub fn clear_reset(&mut self) {
        self.reset_control(false);
    }

    // Enable powerdown mode.
    pub fn enable_powerdown_mode(&mut self) {
        self.power_control(true);
    }

    // Exit powerdown mode.
    pub fn disable_powerdown_mode(&mut self) {
        self.power_control(false);
    }

    pub fn enable_lp_mode(&mut self) {
        self.set_lp_mode(true);
    }

    pub fn disable_lp_mode(&mut self) {
        self.set_lp_mode(false);
    }

    // =====================================================
    // Force a reset of the USB peripheral, exactly like a RESET signaling on
    // the USB. The USB is held in RESET state until software clears this bit.
    // A "USB-RESET" interrupt is generated, if enabled.
    fn reset_control(&self, reset: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !CNTR_FRES;

            if reset {
                *reg |= CNTR_FRES;
            }
        }
    }

    // Turn off all USB-related analog parts if it is required to
    // completely disable the USB peripheral for any reason.
    // When this bit is set, the USB peripheral is disconnected from the
    // transceivers and it cannot be used.
    fn power_control(&self, bool: enable) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !CNTR_PDWN;

            if enable {
                *reg |= CNTR_PDWN;
            }
        }
    }

    // NOTE: Do we care about low power mode??
    // Take another look at this setup.
    fn set_lp_mode(&self, enable: bool) {
        unsafe {
            let mut reg = self.addr();

            *reg & !CNTR_LP_MODE
            if enable {
                *reg |= CNTR_LP_MODE;
            }
        }
    }

    // Sofware must set this bit when the SUSP interrupt is received, which is
    // issued when no traffic is received by the USB peripheral for 3ms.
    pub fn force_suspend(&self) {
        unsafe {
            let mut reg = self.addr();
            *reg |= CNTR_FSUSP;
        }
    }

    // NOTE: Setting FSUSP = 0 has no effect according to the data sheet.
    // Look at the force suspend and how to resume??? fuck who knows.
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

}
