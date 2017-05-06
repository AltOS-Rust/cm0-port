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

use super::DMAChannel;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct IFCR(u32);

impl IFCR {
    pub fn channel_global_interrupt_clear(&mut self, chan: DMAChannel) {
        self.0 |= DMA_CGIF_1 << (4 * (chan as u32));
    }

    pub fn channel_transfer_complete_clear(&mut self, chan: DMAChannel) {
        self.0 |= DMA_CTCIF_1 << (4 * (chan as u32));
    }

    pub fn channel_half_transfer_clear(&mut self, chan: DMAChannel) {
        self.0 |= DMA_CHTIF_1 << (4 * (chan as u32));
    }

    pub fn channel_transfer_error_clear(&mut self, chan: DMAChannel) {
        self.0 |= DMA_CTEIF_1 << (4 * (chan as u32));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_global_interrupt_clear_sets_correct_bit_for_chan_one() {
        let mut ifcr = IFCR(0);
        ifcr.channel_global_interrupt_clear(DMAChannel::One);
        assert_eq!(ifcr.0, 0b1);
    }

    #[test]
    fn channel_global_interrupt_clear_sets_correct_bit_for_chan_two() {
        let mut ifcr = IFCR(0);
        ifcr.channel_global_interrupt_clear(DMAChannel::Two);
        assert_eq!(ifcr.0, 0b1 << 4);
    }

    #[test]
    fn channel_global_interrupt_clear_sets_correct_bit_for_chan_three() {
        let mut ifcr = IFCR(0);
        ifcr.channel_global_interrupt_clear(DMAChannel::Three);
        assert_eq!(ifcr.0, 0b1 << 8);
    }

    #[test]
    fn channel_global_interrupt_clear_sets_correct_bit_for_chan_four() {
        let mut ifcr = IFCR(0);
        ifcr.channel_global_interrupt_clear(DMAChannel::Four);
        assert_eq!(ifcr.0, 0b1 << 12);
    }

    #[test]
    fn channel_global_interrupt_clear_sets_correct_bit_for_chan_five() {
        let mut ifcr = IFCR(0);
        ifcr.channel_global_interrupt_clear(DMAChannel::Five);
        assert_eq!(ifcr.0, 0b1 << 16);
    }

    #[test]
    fn channel_transfer_complete_clear_sets_correct_bit_for_chan_one() {
        let mut ifcr = IFCR(0);
        ifcr.channel_transfer_complete_clear(DMAChannel::One);
        assert_eq!(ifcr.0, 0b1 << 1);
    }

    #[test]
    fn channel_transfer_complete_clear_sets_correct_bit_for_chan_two() {
        let mut ifcr = IFCR(0);
        ifcr.channel_transfer_complete_clear(DMAChannel::Two);
        assert_eq!(ifcr.0, 0b1 << 5);
    }

    #[test]
    fn channel_transfer_complete_clear_sets_correct_bit_for_chan_three() {
        let mut ifcr = IFCR(0);
        ifcr.channel_transfer_complete_clear(DMAChannel::Three);
        assert_eq!(ifcr.0, 0b1 << 9);
    }

    #[test]
    fn channel_transfer_complete_clear_sets_correct_bit_for_chan_four() {
        let mut ifcr = IFCR(0);
        ifcr.channel_transfer_complete_clear(DMAChannel::Four);
        assert_eq!(ifcr.0, 0b1 << 13);
    }

    #[test]
    fn channel_transfer_complete_clear_sets_correct_bit_for_chan_five() {
        let mut ifcr = IFCR(0);
        ifcr.channel_transfer_complete_clear(DMAChannel::Five);
        assert_eq!(ifcr.0, 0b1 << 17);
    }

    #[test]
    fn channel_half_transfer_complete_clear_sets_correct_bit_for_chan_one() {
        let mut ifcr = IFCR(0);
        ifcr.channel_half_transfer_clear(DMAChannel::One);
        assert_eq!(ifcr.0, 0b1 << 2);
    }

    #[test]
    fn channel_half_transfer_complete_clear_sets_correct_bit_for_chan_two() {
        let mut ifcr = IFCR(0);
        ifcr.channel_half_transfer_clear(DMAChannel::Two);
        assert_eq!(ifcr.0, 0b1 << 6);
    }

    #[test]
    fn channel_half_transfer_complete_clear_sets_correct_bit_for_chan_three() {
        let mut ifcr = IFCR(0);
        ifcr.channel_half_transfer_clear(DMAChannel::Three);
        assert_eq!(ifcr.0, 0b1 << 10);
    }

    #[test]
    fn channel_half_transfer_complete_clear_sets_correct_bit_for_chan_four() {
        let mut ifcr = IFCR(0);
        ifcr.channel_half_transfer_clear(DMAChannel::Four);
        assert_eq!(ifcr.0, 0b1 << 14);
    }

    #[test]
    fn channel_half_transfer_complete_clear_sets_correct_bit_for_chan_five() {
        let mut ifcr = IFCR(0);
        ifcr.channel_half_transfer_clear(DMAChannel::Five);
        assert_eq!(ifcr.0, 0b1 << 18);
    }

    #[test]
    fn channel_transfer_error_clear_sets_correct_bit_for_chan_one() {
        let mut ifcr = IFCR(0);
        ifcr.channel_transfer_error_clear(DMAChannel::One);
        assert_eq!(ifcr.0, 0b1 << 3);
    }

    #[test]
    fn channel_transfer_error_clear_sets_correct_bit_for_chan_two() {
        let mut ifcr = IFCR(0);
        ifcr.channel_transfer_error_clear(DMAChannel::Two);
        assert_eq!(ifcr.0, 0b1 << 7);
    }

    #[test]
    fn channel_transfer_error_clear_sets_correct_bit_for_chan_three() {
        let mut ifcr = IFCR(0);
        ifcr.channel_transfer_error_clear(DMAChannel::Three);
        assert_eq!(ifcr.0, 0b1 << 11);
    }

    #[test]
    fn channel_transfer_error_clear_sets_correct_bit_for_chan_four() {
        let mut ifcr = IFCR(0);
        ifcr.channel_transfer_error_clear(DMAChannel::Four);
        assert_eq!(ifcr.0, 0b1 << 15);
    }

    #[test]
    fn channel_transfer_error_clear_sets_correct_bit_for_chan_five() {
        let mut ifcr = IFCR(0);
        ifcr.channel_transfer_error_clear(DMAChannel::Five);
        assert_eq!(ifcr.0, 0b1 << 19);
    }
}
