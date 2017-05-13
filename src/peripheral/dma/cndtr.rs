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

#[derive(Copy, Clone, Debug)]
pub struct CNDTR(u32);

impl CNDTR {
    /* Bits 31:16 Reserved, must be kept at reset value.
     * Bits 15:0 NDT[15:0]: Number of data to transfer
     *
     * Number of data to be transferred (0 up to 65535).
     *
     * This register can only be written when the channel is disabled.
     *   Once the channel is enabled, this register is read-only, indicating the remaining
     *   bytes to be transmitted. This register decrements after each DMA transfer.
     *
     * Once the transfer is completed, this register can either stay at zero or be
     *   reloaded automatically by the value previously programmed if the channel is
     *   configured in circular mode.
     *
     * If this register is zero, no transaction can be served whether the channel is
     *   enabled or not.
     */
    /// Set the number of data to be transferred. Up to 65535.
    pub fn set_ndt(&mut self, num_data: u16) {
        self.0 = num_data as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cndtr_set_ndt_stores_value_correctly() {
        let mut cndtr = CNDTR(0);
        assert_eq!(cndtr.0, 0b0);

        cndtr.set_ndt(65535);
        assert_eq!(cndtr.0, 65535);

        cndtr.set_ndt(5);
        assert_eq!(cndtr.0, 5);
    }

    // TODO: Tests for out of range values?
}
