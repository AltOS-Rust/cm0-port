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

use super::defs::*;

/// The Reload Value Register specifies the start value to load into the SYST_CVR
/// (Current Value Register).
#[derive(Copy, Clone, Debug)]
pub struct RVR(u32);

impl RVR {
    /// Return the reload value of the register.
    pub fn get_reload_value(&self) -> u32 {
        self.0 & RELOAD
    }

    /// Set the reload value of the register. It must be <= 0xFFFFFF or the kernel will panic.
    pub fn set_reload_value(&mut self, value: u32) {
        if value & !RELOAD != 0 {
            panic!("RVR::set_reload_value - the value of the reload register must be <= 0xFFFFFF!");
        }

        self.0 = value;
    }
}
