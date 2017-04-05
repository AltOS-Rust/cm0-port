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

/* This file contains the constants associated with the bit definitions
 * for the registers being used.
 * This is not a complete listing, however, all constants used throughout
 * the program are listed here, there are bit definitions that are listed
 * and not being used, or not listed at all.
*/

// Base address for the USB peripheral.
pub const USB_ADDR: *const u32 = 0x4000_5C00 as *const _;

// ------------------------------------
// EPnR Bit definitions
// ------------------------------------
// Address offset is from: 0x00 to 0x1C
pub const EPnR_EA: u32 = 0b1111;
pub const EPnR_STAT_TX: u32 = 0b11 << 4;
pub const EPnR_DTOG_TX: u32 = 0b1  << 6;
pub const EPnR_CTR_TX:  u32 = 0b1  << 7;
pub const EPnR_EP_KIND: u32 = 0b1  << 8;
pub const EPnR_EP_TYPE: u32 = 0b11 << 9;
pub const EPnR_SETUP:   u32 = 0b11 << 10;
pub const EPnR_STAT_RX: u32 = 0b11 << 12;
pub const EPnR_DTOG_RX: u32 = 0b1  << 14;
pub const EPnR_CTR_RX:  u32 = 0b1  << 15;

// ------------------------------------
// Addresses 0x20 - 0x3F are reserved.
// ------------------------------------

// ------------------------------------
// CNTR Bit definitions
// ------------------------------------
pub const CNTR_OFFSET:   u32 = 0x40;
pub const CNTR_FRES:     u32 = 0b1;
pub const CNTR_PDWN:     u32 = 0b1 << 1;
pub const CNTR_LP_MODE:  u32 = 0b1 << 2;
pub const CNTR_FSUSP:    u32 = 0b1 << 3;
pub const CNTR_RESUME:   u32 = 0b1 << 4;
pub const CNTR_L1RESUME: u32 = 0b1 << 5;
// Bit 6 is Reserved.
pub const CNTR_L1REQM:   u32 = 0b1 << 7;
pub const CNTR_ESOFM:    u32 = 0b1 << 8;
pub const CNTR_SOFM:     u32 = 0b1 << 9;
pub const CNTR_RESETM:   u32 = 0b1 << 10;
pub const CNTR_SUSPM:    u32 = 0b1 << 11;
pub const CNTR_WKUPM:    u32 = 0b1 << 12;
pub const CNTR_ERRM:     u32 = 0b1 << 13;
pub const CNTR_PMAOVRM:  u32 = 0b1 << 14;
pub const CNTR_CTRM:     u32 = 0b1 << 15;

// ------------------------------------
// ISTR Bit definitions
// ------------------------------------
pub const ISTR_OFFSET: u32 = 0x44;
pub const ISTR_RESET:  u32 = 10;
pub const ISTR_SUSP:   u32 = 11;
pub const ISTR_WKUP:   u32 = 12;
pub const ISTR_CTR:    u32 = 15;

// ------------------------------------
// BTABLE Bit definitions
// ------------------------------------
pub const BTABLE_OFFSET: u32 = 0x50;

// ------------------------------------
// BCDR Bit definitions
// ------------------------------------
pub const BCDR_OFFSET: u32 = 0x58;
pub const BCDR_DPPU: u32 = 15;

// ----------------------------------------------
// TODO: Figure out what these are doing and why.
//
// NOTE: Why does this need to be a constant?
pub const POWER_MANAGEMENT:u32 = 0;

pub const USB_BDT_COUNT_RX_BL_SIZE:u32 =15;
pub const USB_BDT_COUNT_RX_BL_NUM_BLOCK:u32 =10;
pub const USB_BDT_COUNT_RX_COUNT_RX_MASK:u32 = 0x1ff;

pub const ISTR_EP_ID_MASK:u32 = 0xf;
pub const ISTR_RESET_FLAG:usize = 19;
pub const ISTR_SUSPEND_FLAG:usize = 20;
pub const ISTR_WAKEUP_FLAG:usize = 21;

pub const RX_COUNT_SIZE:u32 = ((1 << USB_BDT_COUNT_RX_BL_SIZE) | (((USB_OUT_SIZE / 32) - 1) << USB_BDT_COUNT_RX_BL_NUM_BLOCK));
pub const USB_OUT_SIZE:u32 = 64;
//
//Constants used by sram
pub const BDT_SIZE:u32 = 8;

// NOTE: USB_SET_LINE_CODING?? From original Altos.. are these the same??
pub const SRAM_ADDR: *const u32 = 0x20 as *const _;

pub const EPR_STAT_TX_MASK:u32 = 3;
pub const EPR_STAT_RX_MASK:u32 = 3;
pub const EPR_STAT_TX:u32 = 4;

pub const EPR_CONTROL:u32 = 0;
pub const EPR_INT:u32 = 1;
pub const EPR_INPUT:u32 = 5;
pub const EPR_OUTPUT:u32 = 4;

pub const EPR_TYPE_CONTROL:u32 = 1;
pub const EPR_TYPE_INTERRUPT:u32 = 3;
pub const EPR_TYPE_BULK:u32 = 0;
pub const EPR_TYPE_MASK:u32 = 3;

pub const EPR_STAT_RX_VALID:u32 = 3;
pub const EPR_STAT_TX_VALID:u32 = 3;

pub const EPR_STAT_RX_DISABLED:u32 = 0;
pub const EPR_STAT_TX_DISABLED:u32 = 0;

pub const EPR_STAT_RX_NAK:u32 = 2;
pub const EPR_STAT_TX_NAK:u32 = 2;

pub const EPR_INVARIANT:u32 = ((1 << USB_EPR_CTR_RX) |
                                    (USB_EPR_DTOG_RX_WRITE_INVARIANT << USB_EPR_DTOG_RX) |
                                    (USB_EPR_STAT_RX_WRITE_INVARIANT << USB_EPR_STAT_RX) |
                                    (1 << USB_EPR_CTR_TX) |
                                    (USB_EPR_DTOG_TX_WRITE_INVARIANT << USB_EPR_DTOG_TX) |
                                    (USB_EPR_STAT_TX_WRITE_INVARIANT << USB_EPR_STAT_TX));
pub const USB_EPR_INVARIANT_MASK:u32 = ((1 << USB_EPR_CTR_RX) |
                                         (USB_EPR_DTOG_RX_MASK << USB_EPR_DTOG_RX) |
                                         (USB_EPR_STAT_RX_MASK << USB_EPR_DTOG_RX) |
                                         (1 << USB_EPR_CTR_TX) |
                                         (USB_EPR_DTOG_TX_MASK << USB_EPR_DTOG_TX) |
                                         (USB_EPR_STAT_TX_MASK << USB_EPR_STAT_TX));


pub const USB_EPR_PRESERVE_MASK:u32 = ((USB_EPR_EP_TYPE_MASK << USB_EPR_EP_TYPE) |
                                        (1 << USB_EPR_EP_KIND) |
                                        (USB_EPR_EA_MASK << USB_EPR_EA));

pub const USB_EPR_DTOG_TX_MASK:u32 = 0;
pub const USB_EPR_DTOG_RX_MASK:u32 = 0;
pub const USB_EPR_CTR_EP_TYPE_MASK: u32 = 3;
pub const USB_EPR_EA_MASK: u32 = 0xf;
pub const USB_EPR_EP_TYPE_MASK:u32 = 3;

pub const USB_EPR_STAT_RX_WRITE_INVARIANT: u32 = 0;
pub const USB_EPR_STAT_TX_WRITE_INVARIANT: u32 = 0;
pub const USB_EPR_DTOG_RX_WRITE_INVARIANT: u32 = 0;
pub const USB_EPR_DTOG_TX_WRITE_INVARIANT: u32 = 0;

//Constants used by USB
pub const EP_NUM:usize = 4;

pub const EP0: usize = 0;
pub const INT: usize = 1;
pub const OUT: usize = 2;
pub const IN: usize  = 3;
pub const IN2: usize = 4;

pub const BUF_SIZE:usize = 64;
pub const MAX_BUF_SIZE:u32   = 64;
pub const USB_EPR_SETUP:u32 = 11;
pub const USB_EP0_GOT_RX_DATA:u32 = 2;
pub const USB_EP0_GOT_SETUP:u32 = 1;
pub const USB_EP0_GOT_TX_ACK:u32 = 4;

pub const READ_AGAIN:i8 = -1;

