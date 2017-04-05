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

use super::super::Register;

#[derive(Copy, Clone, Debug)]
pub struct DADDR {
    base_addr: *const u32,
}

/*
 * DADDR is stores the address assigned by the host PC for the endpoints to use during transactions
 */

impl Register for DADDR {
    fn new(base_addr: *const u32) -> Self {
        DADDR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        0x4C
    }
}

impl DADDR {
    //TODO: stuff here
}
