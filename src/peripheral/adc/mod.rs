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

// Need module description/doc comment here

mod defs;
mod isr;
mod ier;
mod cr;
mod dr;
mod chselr;

use core::ops::{Deref, DerefMut};
use volatile::Volatile;
use self::isr::ISR;
use self::cr::CR;
use self::ier::IER;
use self::dr::DR;
use self::chselr::CHSELR;
use self::defs::*;
use peripheral::{rcc, gpio};
// use interrupt;

pad_field!(Pad1[0x8]);
pad_field!(Pad2[0x14]);
pad_field!(Pad3[0x2c4]);

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[doc(hidden)]
pub struct RawADC {
    // Interrupt and status register
    isr: ISR,
    // Interrupt enable register
    ier: IER,
    // Control register
    cr: CR,
    cfgr1: u32,
    cfgr2: u32,
    smpr: u32,
    _pad1: Pad1,
    tr: u32,
    chselr: CHSELR,
    chselr: u32,
    _pad2: Pad2,
    // Data register
    dr: DR,
    _pad3: Pad3,
    ccr: u32,

    // Collection of configuration registers?
    // cfgr1: CFGR1,
    // cfgr2: CFGR2,
    // smpr: SMPR,
}

#[derive(Copy, Clone, Debug)]
pub struct ADC(Volatile<RawADC>);

impl Adc {
    // Creates a new Adc object to allow for configuration of the ADC peripheral.
    fn new() -> Self {
        unsafe {
            ADC(Volatile::new(ADC_ADDR as *const _))
        }
    }
}

impl Deref for ADC {
    type Target = RawADC;

    fn deref(&self) -> &Self::Target {
        &*(self.0)
    }
}

impl DerefMut for ADC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *(self.0)
    }
}

impl RawADC {
    pub fn get_calibration(&mut self) -> u16 {
        self.cr.start_adc_calibration();
        // Wait until calibration is finished
        loop {
            if !self.cr.is_adc_calibrating() {
                break;
            }
        }
        // Calibration data can now be read from data register
        self.dr.get_calibration_factor()
    }

    pub fn adc_ready(&mut self) -> bool {
        self.isr.adc_ready()
    }

    pub fn start_adc_conversion(&mut self) {
        self.cr.start_adc_conversion();
    }

    pub fn enable_adc(&mut self) {
        self.cr.enable_adc();
    }

    pub fn disable_adc(&mut self) {
        self.cr.disable_adc();
    }
}

/// Initialize the ADC peripheral.
///
/// Connects the necessary GPIO pins, sets the clock, enables interrupts?
pub fn init() {
    // Notes from ADC_Config in example code:

    // Enable GPIOC clock
    // Enable ADC1 clock

    let rcc = rcc::rcc();
    rcc.enable_peripheral(rcc::Peripheral::ADC);

    // Configure ADC Channel(Which one?) as analog input ===>
    // GPIO pin: 1 or 0?
    // GPIO mode: GPIO_Mode_AN
    // GPIO PuPd: GPIO_PuPd_NOPULL
    // Init GPIO

    gpio::GPIO::enable(gpio::Group::C);
    let mut pa0 = gpio::Port::new(0, gpio::Group::C); // Correct pin?

    // pa0.set_function(gpio::AlternateFunction::One);
    // pa0.set_speed(gpio::Speed::High);
    pa0.set_mode(gpio::Mode::Analog); // Is this right?
    pa0.set_type(gpio::Type::PushPull); // Should this be OpenDrain or PushPull?
    pa0.set_pull(gpio::Pull::Neither);

    let mut adc1 = ADC::new();

    // Functions we need...
    // adc1.set_resolution();
    // adc1.set_continuous_conversion_mode();
    // adc1.set_align();
    // adc1.set_scan_direction();
    // adc1.set_channel_config();
    // adc1.configure_channel();
    // adc1.get_calibration();

    // ADC DeInit (?)
    // Initialize ADC structure (?)

    // Configure ADC options ===>
    // Resolution: 12 bits?
    // Continuous Conversion Mode: Enable
    // ExternalTrigConvEdge (?): None
    // DataAlign: Right
    // ScanDirection: Upward
    // Init ADC

    // ADC channel config (239.5 Cycles as sampling time)

    // ADC Calibration (Get calibration factor for ADC1)
    // Right now, we just ignore the retreived calibration value. Is this correct?
    adc1.get_calibration();

    // Enable ADC Peripheral (ADC1)
    adc1.enable_adc();

    // Wait for ADRDY flag to be set
    loop {
        if adc1.adc_ready() {
            break;
        }
    }

    // ADC start of conversion (ADC1)
    adc1.start_adc_conversion();
}
