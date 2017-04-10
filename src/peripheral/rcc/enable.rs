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

//! This module is used to control the AHBENR (AHB peripheral enable register), which controls the
//! clock to the peripherals controled by the AHB clock.

use super::super::Field;
use super::defs::*;

/// Defines available peripherals.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Peripheral {
    // AHB Peripherals
    TouchSenseController,
    GPIOA,
    GPIOB,
    GPIOC,
    GPIOF,
    CRC,
    FLITF,
    SRAMInterface,
    DMA,
    DMA2,

    // APB1 Peripherals
    CEC,
    DAC,
    PowerInterface,
    ClockRecoverySystem,
    CAN,
    USB,
    I2C1,
    I2C2,
    USART2,
    USART3,
    USART4,
    USART5,
    SPI2,
    WindowWatchdog,
    TIM2,
    TIM3,
    TIM6,
    TIM7,
    TIM14,

    // APB2 Peripherals
    MCUDebug,
    TIM1,
    TIM15,
    TIM16,
    TIM17,
    USART1,
    USART6,
    USART7,
    USART8,
    SPI1,
    ADC,
    SysCfgComp,
}

impl Field for Peripheral {
    fn mask(&self) -> u32 {
        match *self {
            // AHB Peripherals
            Peripheral::TouchSenseController => TSCEN,
            Peripheral::GPIOA => IOPAEN,
            Peripheral::GPIOB => IOPBEN,
            Peripheral::GPIOC => IOPCEN,
            Peripheral::GPIOF => IOPFEN,
            Peripheral::CRC => CRCEN,
            Peripheral::FLITF => FLITFEN,
            Peripheral::SRAMInterface => SRAMEN,
            Peripheral::DMA => DMAEN,
            Peripheral::DMA2 => DMA2EN,

            // APB1 Peripherals
            Peripheral::CEC => CECEN,
            Peripheral::DAC => DACEN,
            Peripheral::PowerInterface => PWREN,
            Peripheral::ClockRecoverySystem => CRSEN,
            Peripheral::CAN => CANEN,
            Peripheral::USB => USBEN,
            Peripheral::I2C1 => I2C1EN,
            Peripheral::I2C2 => I2C2EN,
            Peripheral::USART2 => USART2EN,
            Peripheral::USART3 => USART3EN,
            Peripheral::USART4 => USART4EN,
            Peripheral::USART5 => USART5EN,
            Peripheral::SPI2 => SPI2EN,
            Peripheral::WindowWatchdog => WWDGEN,
            Peripheral::TIM2 => TIM2EN,
            Peripheral::TIM3 => TIM3EN,
            Peripheral::TIM6 => TIM6EN,
            Peripheral::TIM7 => TIM7EN,
            Peripheral::TIM14 => TIM14EN,

            // APB2 Peripherals
            Peripheral::MCUDebug => DBGMCUEN,
            Peripheral::TIM1 => TIM1EN,
            Peripheral::TIM15 => TIM15EN,
            Peripheral::TIM16 => TIM16EN,
            Peripheral::TIM17 => TIM17EN,
            Peripheral::USART1 => USART1EN,
            Peripheral::USART6 => USART6EN,
            Peripheral::USART7 => USART7EN,
            Peripheral::USART8 => USART8EN,
            Peripheral::SPI1 => SPI1EN,
            Peripheral::ADC => ADCEN,
            Peripheral::SysCfgComp => SYSCFGCOMPEN,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AHBENR(u32);

impl AHBENR {
    pub fn get_enable(&self, peripheral: Peripheral) -> bool {
        if !self.serves_peripheral(peripheral) {
            panic!("AHBENR::get_enable - this register does not control the specified peripheral!");
        }
        let mask = peripheral.mask();

        self.0 & mask != 0
    }

    pub fn set_enable(&mut self, enable: bool, peripheral: Peripheral) {
        if !self.serves_peripheral(peripheral) {
            panic!("AHBENR::enable - This register does not control the specified peripheral!");
        }
        let mask = peripheral.mask();

        self.0 &= !mask;
        if enable {
            self.0 |= mask;
        }
    }

    pub fn serves_peripheral(&self, peripheral: Peripheral) -> bool {
        match peripheral {
            Peripheral::TouchSenseController | Peripheral::GPIOA |
            Peripheral::GPIOB | Peripheral::GPIOC | Peripheral::GPIOF |
            Peripheral::CRC | Peripheral::FLITF | Peripheral::SRAMInterface |
            Peripheral::DMA | Peripheral::DMA2 => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct APBENR1(u32);

impl APBENR1 {
    pub fn get_enable(&self, peripheral: Peripheral) -> bool {
        if !self.serves_peripheral(peripheral) {
            panic!("APBENR1::get_enable - this register does not control the specified peripheral!");
        }
        let mask = peripheral.mask();

        self.0 & mask != 0
    }

    pub fn set_enable(&mut self, enable: bool, peripheral: Peripheral) {
        if !self.serves_peripheral(peripheral) {
            panic!("APBENR1::enable - This register does not control the specified peripheral!");
        }
        let mask = peripheral.mask();

        self.0 &= !mask;
        if enable {
            self.0 |= mask;
        }
    }

    pub fn serves_peripheral(&self, peripheral: Peripheral) -> bool {
        match peripheral {
            Peripheral::CEC | Peripheral::DAC | Peripheral::PowerInterface |
            Peripheral::ClockRecoverySystem | Peripheral::CAN | Peripheral::USB |
            Peripheral::I2C1 | Peripheral::I2C2 | Peripheral::USART2 |
            Peripheral::USART3 | Peripheral::USART4 | Peripheral::USART5 |
            Peripheral::SPI2 | Peripheral::WindowWatchdog | Peripheral::TIM2 |
            Peripheral::TIM3 | Peripheral::TIM6 | Peripheral::TIM7 | Peripheral::TIM14 => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct APBENR2(u32);

impl APBENR2 {
    pub fn get_enable(&self, peripheral: Peripheral) -> bool {
        if !self.serves_peripheral(peripheral) {
            panic!("APBENR2::get_enable - this register does not control the specified peripheral!");
        }
        let mask = peripheral.mask();

        self.0 & mask != 0
    }

    pub fn set_enable(&mut self, enable: bool, peripheral: Peripheral) {
        if !self.serves_peripheral(peripheral) {
            panic!("APBENR2::enable - This register does not control the specified peripheral!");
        }
        let mask = peripheral.mask();

        self.0 &= !mask;
        if enable {
            self.0 |= mask;
        }
    }

    pub fn serves_peripheral(&self, peripheral: Peripheral) -> bool {
        match peripheral {
            Peripheral::MCUDebug | Peripheral::TIM1 | Peripheral::TIM15 |
            Peripheral::TIM16 | Peripheral::TIM17 | Peripheral::USART1 |
            Peripheral::USART6 | Peripheral::USART7 | Peripheral::USART8 |
            Peripheral::SPI1 | Peripheral::ADC | Peripheral::SysCfgComp => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ahbenr_get_enable() {
        // GPIO Group A starts enabled
        let ahbenr = AHBENR(0b1 << 17);

        assert_eq!(ahbenr.get_enable(Peripheral::GPIOA), true);
        assert_eq!(ahbenr.get_enable(Peripheral::GPIOB), false);
    }

    #[test]
    #[should_panic]
    fn test_ahbenr_get_enable_unserved_peripheral_panics() {
        let ahbenr = AHBENR(0);

        ahbenr.get_enable(Peripheral::USART1);
    }

    #[test]
    fn test_ahbenr_set_enable_on() {
        let mut ahbenr = AHBENR(0);

        ahbenr.set_enable(true, Peripheral::GPIOB);
        assert_eq!(ahbenr.0, 0b1 << 18);
    }

    #[test]
    fn test_ahbenr_set_enable_off() {
        // CRC starts enabled
        let mut ahbenr = AHBENR(0b1 << 6);

        ahbenr.set_enable(false, Peripheral::CRC);
        assert_eq!(ahbenr.0, 0);
    }

    #[test]
    fn test_ahbenr_set_enable_on_multiple_peripherals_doesnt_change_settings() {
        let mut ahbenr = AHBENR(0);

        ahbenr.set_enable(true, Peripheral::GPIOA);
        assert_eq!(ahbenr.0, 0b1 << 17);

        ahbenr.set_enable(true, Peripheral::CRC);
        assert_eq!(ahbenr.0, 0b1 << 6 | 0b1 << 17);
    }

    #[test]
    fn test_ahbenr_set_enable_off_multiple_peripherals_doesnt_change_settings() {
        // SRAM and GPIOB start enabled
        let mut ahbenr = AHBENR(0b1 << 2 | 0b1 << 18);

        ahbenr.set_enable(false, Peripheral::GPIOB);
        assert_eq!(ahbenr.0, 0b1 << 2);

        ahbenr.set_enable(false, Peripheral::SRAMInterface);
        assert_eq!(ahbenr.0, 0);
    }

    #[test]
    #[should_panic]
    fn test_ahbenr_set_enable_unserved_peripheral_panics() {
        let mut ahbenr = AHBENR(0);

        ahbenr.set_enable(true, Peripheral::USART1);
    }

    #[test]
    fn test_apbenr1_get_enable() {
        // USB starts enabled
        let apbenr1 = APBENR1(0b1 << 23);

        assert_eq!(apbenr1.get_enable(Peripheral::USB), true);
        assert_eq!(apbenr1.get_enable(Peripheral::CEC), false);
    }

    #[test]
    #[should_panic]
    fn test_apbenr1_get_enable_unserved_peripheral_panics() {
        let apbenr1 = APBENR1(0);

        apbenr1.get_enable(Peripheral::GPIOA);
    }

    #[test]
    fn test_apbenr1_set_enable_on() {
        let mut apbenr1 = APBENR1(0);

        apbenr1.set_enable(true, Peripheral::USART2);
        assert_eq!(apbenr1.0, 0b1 << 17);
    }

    #[test]
    fn test_apbenr1_set_enable_off() {
        // SPI2 starts enabled
        let mut apbenr1 = APBENR1(0b1 << 14);

        apbenr1.set_enable(false, Peripheral::SPI2);
        assert_eq!(apbenr1.0, 0);
    }

    #[test]
    fn test_apbenr1_set_enable_on_multiple_peripherals_doesnt_change_settings() {
        let mut apbenr1 = APBENR1(0);

        apbenr1.set_enable(true, Peripheral::USART2);
        assert_eq!(apbenr1.0, 0b1 << 17);

        apbenr1.set_enable(true, Peripheral::SPI2);
        assert_eq!(apbenr1.0, 0b1 << 14 | 0b1 << 17);
    }

    #[test]
    fn test_apbenr1_set_enable_off_multiple_peripherals_doesnt_change_settings() {
        // TIM3 and PWR start enabled
        let mut apbenr1 = APBENR1(0b1 << 1 | 0b1 << 28);

        apbenr1.set_enable(false, Peripheral::PowerInterface);
        assert_eq!(apbenr1.0, 0b1 << 1);

        apbenr1.set_enable(false, Peripheral::TIM3);
        assert_eq!(apbenr1.0, 0);
    }

    #[test]
    #[should_panic]
    fn test_apbenr1_set_enable_unserved_peripheral_panics() {
        let mut apbenr1 = APBENR1(0);

        apbenr1.set_enable(true, Peripheral::GPIOA);
    }

    #[test]
    fn test_apbenr2_get_enable() {
        // USART1 starts enabled
        let apbenr2 = APBENR2(0b1 << 14);

        assert_eq!(apbenr2.get_enable(Peripheral::USART1), true);
        assert_eq!(apbenr2.get_enable(Peripheral::SPI1), false);
    }

    #[test]
    #[should_panic]
    fn test_apbenr2_get_enable_unserved_peripheral_panics() {
        let apbenr2 = APBENR2(0);

        apbenr2.get_enable(Peripheral::GPIOA);
    }

    #[test]
    fn test_apbenr2_set_enable_on() {
        let mut apbenr2 = APBENR2(0);

        apbenr2.set_enable(true, Peripheral::ADC);
        assert_eq!(apbenr2.0, 0b1 << 9);
    }

    #[test]
    fn test_apbenr2_set_enable_off() {
        // USART1 starts enabled
        let mut apbenr2 = APBENR2(0b1 << 14);

        apbenr2.set_enable(false, Peripheral::USART1);
        assert_eq!(apbenr2.0, 0);
    }

    #[test]
    fn test_apbenr2_set_enable_on_multiple_peripherals_doesnt_change_settings() {
        let mut apbenr2 = APBENR2(0);

        apbenr2.set_enable(true, Peripheral::TIM17);
        assert_eq!(apbenr2.0, 0b1 << 18);

        apbenr2.set_enable(true, Peripheral::SysCfgComp);
        assert_eq!(apbenr2.0, 0b1 | 0b1 << 18);
    }

    #[test]
    fn test_apbenr2_set_enable_off_multiple_peripherals_doesnt_change_settings() {
        // DBGMCU and USART6 start enabled
        let mut apbenr2 = APBENR2(0b1 << 5 | 0b1 << 22);

        apbenr2.set_enable(false, Peripheral::MCUDebug);
        assert_eq!(apbenr2.0, 0b1 << 5);

        apbenr2.set_enable(false, Peripheral::USART6);
        assert_eq!(apbenr2.0, 0);
    }

    #[test]
    #[should_panic]
    fn test_apbenr2_set_enable_unserved_peripheral_panics() {
        let mut apbenr2 = APBENR2(0);

        apbenr2.set_enable(true, Peripheral::GPIOA);
    }
}
