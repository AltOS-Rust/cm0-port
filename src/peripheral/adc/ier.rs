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
pub struct IER(u32);

// Note: All of these registers are only allowed to be set by software when ADSTART=0

impl IER {
    /*
    OVRIE: Overrun interrupt enable
    This is cleared and set by software to enable/disable overrun interrupt.
    0: Overrun interrupt disabled
    1: Overrun interrupt enabled
    */
    pub fn set_overrun_interrupt(&mut self, enable: bool) {
        self.0 &= !(IER_OVRIE);

        if enable {
            self.0 |= IER_OVRIE;
        }
    }

    /*
    EOSEQIE: End of conversion sequence interrupt enable
    This is set/cleared by software to enable/disable the end of sequence of conversion interrupt.
    0: EOSEQ interrupt disabled
    1: EOSEQ Interrupt enabled
    */
    pub fn set_end_of_conversion_sequence_interrupt(&mut self, enable: bool) {
        self.0 &= !(IER_EOSEQIE);

        if enable {
            self.0 |= IER_EOSEQIE;
        }
    }

    /*
    EOCIE: End of conversion interrupt enabled
    Set and cleared by software to enable/disable end of conversion interrupt.
    0: EOC interrupt disabled
    1: EOC interrupt enabled
    */
    pub fn set_end_of_conversion_interrupt(&mut self, enable: bool) {
        self.0 &= !(IER_EOCIE);

        if enable {
            self.0 |= IER_EOCIE;
        }
    }

    /*
    EOSMPIE: End of sampling flag interrupt enable
    Set and cleared by software to enable/disable the end of sampling phase interrupt.
    0: EOSMP interrupt disabled
    1: EOSMP interrupt enabled
    */
    pub fn set_end_of_sampling_interrupt(&mut self, enable: bool) {
        self.0 &= !(IER_EOSMPIE);

        if enable {
            self.0 |= IER_EOSMPIE;
        }
    }

    /*
    ADRDYIE: ADC ready interrupt enable
    Set and cleared by software to enable/disable the ADC Ready interrupt.
    0: ADRDY interrupt disabled
    1: ADRDY interrupt enabled
    */
    pub fn set_adc_ready_interrupt(&mut self, enable: bool) {
        self.0 &= !(IER_ADRDYIE);

        if enable {
            self.0 |= IER_ADRDYIE;
        }
    }
}
