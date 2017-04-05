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

pub const NVIC_ADDR: *const u32 = 0xE000E100 as *const _;

// Interrupt Numbers
pub const NVIC_WWDG_INT: isize = 0;
pub const NVIC_PVDVDDIO2_INT: isize = 1;
pub const NVIC_RTC_INT: isize = 2;
pub const NVIC_FLASH_INT: isize = 3;
pub const NVIC_RCCCRS_INT: isize = 4;
pub const NVIC_EXTI01_INT: isize = 5;
pub const NVIC_EXTI23_INT: isize = 6;
pub const NVIC_EXTI415_INT: isize = 7;
pub const NVIC_TSC_INT: isize = 8;
pub const NVIC_DMACH1_INT: isize = 9;
pub const NVIC_DMACH23_INT: isize = 10;
pub const NVIC_DMACH4PLUS_INT: isize = 11;
pub const NVIC_ADCCOMP_INT: isize = 12;
pub const NVIC_TIM1BRKUP_INT: isize = 13;
pub const NVIC_TIM1CC_INT: isize = 14;
pub const NVIC_TIM2_INT: isize = 15;
pub const NVIC_TIM3_INT: isize = 16;
pub const NVIC_TIM6_INT: isize = 17;
pub const NVIC_TIM7_INT: isize = 18;
pub const NVIC_TIM14_INT: isize = 19;
pub const NVIC_TIM15_INT: isize = 20;
pub const NVIC_TIM16_INT: isize = 21;
pub const NVIC_TIM17_INT: isize = 22;
pub const NVIC_I2C1_INT: isize = 23;
pub const NVIC_I2C2_INT: isize = 24;
pub const NVIC_SPI1_INT: isize = 25;
pub const NVIC_SPI2_INT: isize = 26;
pub const NVIC_USART1_INT: isize = 27;
pub const NVIC_USART2_INT: isize = 28;
pub const NVIC_USART3PLUS_INT: isize = 29;
pub const NVIC_CECCAN_INT: isize = 30;
pub const NVIC_USB_INT: isize = 31;

// IPR
pub const IPR_PRIORITY_FIELD_MASK: u32 = 0b11 << 6;

pub const IPR_PRIORITY_HIGHEST: u32 = 0b00 << 6;
pub const IPR_PRIORITY_HIGH: u32 = 0b01 << 6;
pub const IPR_PRIORITY_LOW: u32 = 0b10 << 6;
pub const IPR_PRIORITY_LOWEST: u32 = 0b11 << 6;
