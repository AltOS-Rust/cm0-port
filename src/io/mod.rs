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

//! This module handles input and output through the serial port.
//!
//! It implements print formatting for debug and for non-debug purposes.
//! Serial and DebugSerial types provide interfaces for printing characters
//! to the serial port.
//!
//! This module contains implementations of helper macros for print and println.

mod serial;

pub use io::serial::imp::*;
pub use self::dma::*;

#[cfg(not(feature="serial"))]
mod imp {
    use core::fmt::Arguments;
    #[no_mangle]
    #[doc(hidden)]
    pub fn debug_fmt(_args: Arguments) {
        // Stub
    }
}

