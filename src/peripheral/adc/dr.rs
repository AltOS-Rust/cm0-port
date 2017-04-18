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

#[derive(Copy, Clone, Debug)]
pub struct DR(u32);

impl DR {
    /*
    DATA[15:0]: Converted data
    These bits are read only. They contain the conversion result from the last converted channel.
    Data may be left or right aligned.
    Just after a calibration is complete, DATA[6:0] contains the calibration factor.
    */
    // Make this u16?
    pub fn get_converted_data(&self) -> u16 {
        self.0 as u16
    }

    // Calibration factor is put in data register [6:0] at end of calibration
    pub fn get_calibration_factor(&self) -> u16 {
        (self.0 & 0x7F) as u16
    }
}
