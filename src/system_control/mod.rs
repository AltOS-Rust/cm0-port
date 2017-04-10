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

//! This module provides system implementation information and allows
//! configuration control and reporting of system exceptions.

mod icsr;
mod defs;

use core::ops::{Deref, DerefMut};
use ::volatile::Volatile;
use self::icsr::ICSR;
use self::defs::*;

/// Returns instance of the System Control Block.
pub fn scb() -> SCB {
    SCB::scb()
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[doc(hidden)]
pub struct RawSCB {
    cpuid: u32,
    icsr: ICSR,
    reserved1: u32,
    aircr: u32,
    scr: u32,
    ccr: u32,
    reserved2: u32,
    shpr2: u32,
    shpr3: u32,
}

/// System Control Block
#[derive(Copy, Clone, Debug)]
pub struct SCB(Volatile<RawSCB>);

impl SCB {
    fn scb() -> Self {
        unsafe {
            SCB(Volatile::new(SCB_ADDR as *const _))
        }
    }
}

impl Deref for SCB {
    type Target = RawSCB;

    fn deref(&self) -> &Self::Target {
        &*(self.0)
    }
}


impl DerefMut for SCB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *(self.0)
    }
}

impl RawSCB {
    /// Trigger a pend_sv exception.
    ///
    /// PendSV signals to the operating system that a context switch should occur.
    pub fn set_pend_sv(&mut self) {
        self.icsr.set_pend_sv();
    }

    /// Clear the pend_sv exception.
    pub fn clear_pend_sv(&mut self) {
        self.icsr.clear_pend_sv();
    }
}
