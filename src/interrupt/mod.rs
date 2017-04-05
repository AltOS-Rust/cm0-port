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

//! This module defines interrupt behavior.

mod defs;
mod enable;
mod pending;
mod priority;

use volatile::Volatile;
use self::enable::{ISER, ICER};
use self::pending::{ISPR, ICPR};
use self::priority::IPR;
use self::defs::*;
use core::ops::{Deref, DerefMut};
pub use self::priority::Priority;

// Defines all the perpherials that have interrupts.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Hardware {
    Wwdg = NVIC_WWDG_INT,
    Pvdvddio2 = NVIC_PVDVDDIO2_INT,
    Rtc = NVIC_RTC_INT,
    Flash = NVIC_FLASH_INT,
    Rcccrs = NVIC_RCCCRS_INT,
    Exti01 = NVIC_EXTI01_INT,
    Exti23 = NVIC_EXTI23_INT,
    Exti415 = NVIC_EXTI415_INT,
    Tsc = NVIC_TSC_INT,
    Dmach1 = NVIC_DMACH1_INT,
    Dmach23 = NVIC_DMACH23_INT,
    Dmach4Plus = NVIC_DMACH4PLUS_INT,
    Adccomp = NVIC_ADCCOMP_INT,
    Tim1Brkup = NVIC_TIM1BRKUP_INT,
    Tim1cc = NVIC_TIM1CC_INT,
    Tim2 = NVIC_TIM2_INT,
    Tim3 = NVIC_TIM3_INT,
    Tim6 = NVIC_TIM6_INT,
    Tim7 = NVIC_TIM7_INT,
    Tim14 = NVIC_TIM14_INT,
    Tim15 = NVIC_TIM15_INT,
    Tim16 = NVIC_TIM16_INT,
    Tim17 = NVIC_TIM17_INT,
    I2C1 = NVIC_I2C1_INT,
    I2C2 = NVIC_I2C2_INT,
    Spi1 = NVIC_SPI1_INT,
    Spi2 = NVIC_SPI2_INT,
    Usart1 = NVIC_USART1_INT,
    Usart2 = NVIC_USART2_INT,
    Usart3Plus = NVIC_USART3PLUS_INT,
    Ceccan = NVIC_CECCAN_INT,
    Usb = NVIC_USB_INT,
}

/// Get an instance of the nested vector interrupt control.
pub fn nvic() -> Nvic {
    Nvic::new()
}

pad_field!(PadSmall[0x7C]);
pad_field!(PadLarge[0x17C]);

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[doc(hidden)]
pub struct RawNvic {
    iser: ISER,
    _pad0: PadSmall,
    icer: ICER,
    _pad1: PadSmall,
    ispr: ISPR,
    _pad2: PadSmall,
    icpr: ICPR,
    _pad3: PadLarge,
    ipr: [IPR; 8],
}

/// Controls the interrupt vectors for enabling/disabling interrupts for the peripherals.
pub struct Nvic(Volatile<RawNvic>);

impl Nvic {
    fn new() -> Self {
        unsafe { Nvic(Volatile::new(NVIC_ADDR as *const _)) }
    }
}

impl Deref for Nvic {
    type Target = RawNvic;

    fn deref(&self) -> &Self::Target {
        &*(self.0)
    }
}

impl DerefMut for Nvic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *(self.0)
    }
}

impl RawNvic {
    /// Enable the interrupt for the specified peripheral.
    pub fn enable_interrupt(&mut self, hardware: Hardware) {
        self.iser.enable_interrupt(hardware);
    }

    /// Disable the interrupt for the specified peripheral.
    pub fn disable_interrupt(&mut self, hardware: Hardware) {
        self.icer.disable_interrupt(hardware);
    }

    /// Check if the interrupt for the peripheral is enabled.
    pub fn interrupt_is_enabled(&self, hardware: Hardware) -> bool {
        self.iser.interrupt_is_enabled(hardware)
    }

    /// Cause an interrupt for the specified peripheral to be set pending.
    ///
    /// If the interrupt is enabled, the interrupt handler will be called.
    /// Otherwise, no interrupt will be generated until the interrupt is enabled
    /// for the specified peripheral.
    pub fn set_pending(&mut self, hardware: Hardware) {
        self.ispr.set_pending(hardware);
    }

    /// Clear the pending interrupt for the specified peripheral.
    pub fn clear_pending(&mut self, hardware: Hardware) {
        self.icpr.clear_pending(hardware);
    }

    /// Check if interrupt is pending for the specified peripheral.
    pub fn interrupt_is_pending(&self, hardware: Hardware) -> bool {
        self.ispr.interrupt_is_pending(hardware)
    }

    /// Set the priority of the interrupt for the specified peripheral.
    pub fn set_priority(&mut self, priority: Priority, hardware: Hardware) {
        let interrupt = hardware as u8;
        let ipr_offset = interrupt / 4;
        let priority_offset = interrupt % 4;
        self.ipr[ipr_offset as usize].set_priority(priority, priority_offset);
    }

    /// Get the priority of the interrupt for the specified peripheral.
    pub fn get_priority(&self, hardware: Hardware) -> Priority {
        let interrupt = hardware as u8;
        let ipr_offset = interrupt / 4;
        let priority_offset = interrupt % 4;
        self.ipr[ipr_offset as usize].get_priority(priority_offset)
    }
}
