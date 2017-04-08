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

//! This module handles the memory mapped peripherals that are a part of the Cortex-M0. Submodules
//! will handle the more specific details of each peripheral.
pub mod rcc;
pub mod gpio;
pub mod systick;
#[cfg(feature="serial")]
pub mod usart;

use volatile::Volatile;

#[macro_export]
macro_rules! pad_field {
    ($name:ident[$N:expr]) => {
        struct $name([u8; $N]);
        impl Clone for $name {
            fn clone(&self) -> $name {
                $name(self.0)
            }
        }
        impl Copy for $name {}
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                write!(f, "Padding {{ {} bytes }}", self.0.len())
            }
        }
    }
}

/// Defines a bit field within a register.
pub trait Field {
    /// Return the bit mask for the register bit field.
    fn mask(&self) -> u32;
}
