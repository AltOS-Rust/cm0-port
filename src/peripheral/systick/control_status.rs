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

pub enum ClockSource {
    Reference,
    Processor,
}

/// The control and status register for the SysTick timer.
#[derive(Copy, Clone, Debug)]
pub struct CSR(u32);

impl CSR {
    pub fn set_enable(&mut self, enable: bool) {
        if enable {
            self.0 |= ENABLE;
        }
        else {
            self.0 &= !ENABLE;
        }
    }

    pub fn set_interrupt(&mut self, enable: bool) {
        if enable {
            self.0 |= TICKINT;
        }
        else {
            self.0 &= !TICKINT;
        }
    }

    pub fn set_source(&mut self, source: ClockSource) {
        match source {
            ClockSource::Reference => self.0 &= !CLKSOURCE,
            ClockSource::Processor => self.0 |= CLKSOURCE,
        };
    }

    /// Returns true if the counter has reached zero since the last time it was checked.
    pub fn did_underflow(&self) -> bool {
        (self.0 & COUNTFLAG) != 0
    }
}
