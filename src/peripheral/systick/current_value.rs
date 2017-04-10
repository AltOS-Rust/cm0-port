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
pub struct CVR(u32);

impl CVR {
    pub fn get_current_value(&self) -> u32 {
        self.0 & CURRENT
    }

    pub fn clear_current_value(&mut self) {
        // A write to this register clears its value to 0
        self.0 = CLEAR_VALUE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cvr_get_current_value() {
        let cvr = CVR(0xFFFF);

        assert_eq!(cvr.get_current_value(), 0xFFFF);
    }

    #[test]
    fn test_cvr_clear_current_value() {
        // Clearing the current value is done by writing any value the the register (since the
        // register is incremented by hardware) so it doesn't really matter what value we write to
        // the register.
        //
        // The implementation shouldn't matter, but if it ever does change this test could fail if
        // the value written to the register happens to be the same as what it is initialzied to.
        // For this reason we initialize the register to the negation of the constant used to clear
        // it, this should hopefully make this test a bit more sturdy.
        let mut cvr = CVR(!CLEAR_VALUE);
        let old_value = cvr.get_current_value();

        cvr.clear_current_value();
        assert!(cvr.get_current_value() != old_value);
    }
}
